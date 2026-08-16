# broken-app: поиск ошибок и оптимизация

Проект исправлен в рамках проектной работы модуля 5. Исходное поведение сверялось с `reference-app`; файлы эталонного проекта не изменялись.

## Окружение

- Windows 11, `x86_64-pc-windows-msvc`;
- Rust stable 1.97.0;
- Rust nightly 1.100.0-nightly от 2026-08-16;
- Miri 0.1.0;
- LLDB из CodeLLDB 1.12.2;
- Criterion 0.5.1.

Исходные архивы и состояние эталона зафиксированы в `artifacts/environment.txt` и `artifacts/reference-app.sha256`. Неизменённый эталон: [reference-app.zip](https://code.s3.yandex.net/middle-rust-blockchain/reference-app.zip).

## Найденные дефекты

| Участок | Проблема | Как обнаружено | Исправление |
|---|---|---|---|
| `sum_even` | `0..=len` и `get_unchecked` читали за границей среза | падающий тест, Miri | безопасный проход по итератору |
| `leak_buffer` | `Box::into_raw` без восстановления владельца | Miri сообщил утечку 5 байт | подсчёт прямо по входному срезу |
| `use_after_free` | чтение сырого указателя после `Box::from_raw` и `drop` | Miri | значение читается только при живом владельце, `unsafe` удалён |
| `average_positive` | сумма и делитель учитывали отрицательные значения | тест и LLDB | фильтрация положительных значений |
| `normalize` | удалялся только обычный пробел | регрессионный тест | поддержаны все Unicode whitespace |
| `race_increment` | общий `static mut` изменялся без синхронизации | тест: 45 046 вместо 200 000 | `AtomicU64` и ожидание всех потоков |

Для каждого дефекта добавлен или расширен тест в `tests/integration.rs`.

## Профилирование и оптимизации

Исходный текстовый профиль показал основные затраты:

| Нагрузка | Время | Доля |
|---|---:|---:|
| `slow_dedup`, 200 вызовов | 882,900 мс | 48,73% |
| `slow_fib(32)`, 100 вызовов | 637,231 мс | 35,17% |
| `normalize`, 2 000 вызовов | 291,662 мс | 16,10% |

Сделаны две группы оптимизаций:

1. Алгоритмические:
   - рекурсивный Fibonacci `O(2^n)` заменён на цикл `O(n)`;
   - поиск дублей с сортировкой после каждой вставки заменён на одну сортировку и `Vec::dedup`, итоговая сложность `O(n log n)`.
2. Работа с памятью:
   - для ASCII-нормализации используется один заранее выделяемый байтовый буфер;
   - из `average_positive` удалён временный `Vec`.

Criterion, средняя оценка:

| Benchmark | До | После | Ускорение |
|---|---:|---:|---:|
| `normalize_51k` | 272,477 мкс | 75,467 мкс | 3,61× |
| `fib_32` | 15,846 мс | 36,010 нс | 440 057× |
| `dedup_2k` | 9,962 мс | 25,642 мкс | 388,50× |

Сырые Criterion JSON находятся в `artifacts/benchmarks/before` и `artifacts/benchmarks/after`. Исходный commit и финальный commit измерены последовательно в одном Criterion target-каталоге, чтобы условия запуска были одинаковыми. Текстовые профили лежат в `artifacts/profiles`.

## Проверка на Windows

```powershell
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
cargo +nightly miri test --locked
```

Сравнение с сохранённым Criterion baseline:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/compare.ps1
```

Текстовый профиль:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/profile.ps1
```

## Linux-only проверки

В предоставленном Windows-окружении WSL не установлен. Поэтому `Valgrind`, `perf`, Linux ASan/TSan и flamegraph фактически не запускались; это не заменено фиктивными результатами. Проект не содержит FFI, поэтому Valgrind проверяет нативный бинарь на утечки и ошибки памяти.

После установки WSL/Linux команды воспроизводятся так:

```bash
./scripts/valgrind.sh
./scripts/sanitize.sh address
./scripts/sanitize.sh thread
./scripts/profile.sh
```

Подробности попыток запуска инструментов находятся в `artifacts/tool-limitations.md`.

На GitHub Linux-проверки воспроизводятся workflow-файлом `.github/workflows/verification.yml`. Он сохраняет логи Miri, ASan, TSan и Valgrind, а также Callgrind-отчёт и PNG-граф профиля.

## История изменений

- `3f8d0db` — исходный `broken-app` без изменений;
- `c4bee92` — воспроизводимые Criterion benchmarks;
- `87fc207` — регрессионные тесты и текстовый профиль;
- `cb10600` — исправления UB, утечек, логики и гонки;
- `c479c23` — алгоритмические и микрооптимизации.
