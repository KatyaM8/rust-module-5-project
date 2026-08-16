# broken-app: поиск ошибок и оптимизация

Проект исправлен в рамках проектной работы модуля 5. Исходное поведение сверялось с `reference-app`; файлы эталонного проекта не изменялись.

## Окружение

- Windows 11, `x86_64-pc-windows-msvc`;
- Rust stable 1.97.0;
- Rust nightly 1.100.0-nightly от 2026-08-16;
- Miri 0.1.0;
- LLDB из CodeLLDB 1.12.2;
- Criterion 0.5.1;
- Ubuntu (`ubuntu-latest` в GitHub Actions) для Valgrind, Callgrind, ASan и TSan.

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

Так как локальный WSL недоступен, Linux-проверки выполнены в GitHub Actions на Ubuntu. Workflow прошёл полностью: Miri — 12 тестов, ASan и TSan — ключевые регрессионные тесты, Valgrind — интеграционный бинарь и `demo`. Valgrind сообщил `0 errors` и `0 bytes definitely lost` в обоих запусках. Проект не содержит FFI или C-библиотек, поэтому проверялись нативные Rust-бинари.

Полные результаты сохранены в `artifacts/linux`: журналы Miri, ASan, TSan и Valgrind, данные Callgrind, текстовый отчёт и `profile-callgraph.png`. Запуск также виден на странице [Linux verification](https://github.com/KatyaM8/rust-module-5-project/actions/workflows/verification.yml).

После установки WSL/Linux команды воспроизводятся так:

```bash
./scripts/valgrind.sh
./scripts/sanitize.sh address
./scripts/sanitize.sh thread
./scripts/profile.sh
```

Локальные ограничения Windows отдельно описаны в `artifacts/tool-limitations.md`; они не относятся к успешно выполненным Linux-проверкам.

## История изменений

- `f4cc48b` — исходный `broken-app` без изменений;
- `ea82b5d` — воспроизводимые Criterion benchmarks;
- `b39e6a6` — регрессионные тесты и текстовый профиль;
- `4bce1d5` — исправления UB, утечек, логики и гонки;
- `94ceaad` — алгоритмические и микрооптимизации;
- `7a21228` — итоговый отчёт и воспроизводимые команды;
- `e8d1a31` — реальные отчёты Linux-проверок.
