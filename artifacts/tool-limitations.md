# Среды выполнения и ограничения

Результаты ниже не симулировались.

## Linux-проверки

`wsl.exe --list --verbose` сообщил, что локальный WSL не установлен. Поэтому Valgrind, Callgrind, Linux ASan и TSan запущены на Ubuntu через `.github/workflows/verification.yml`. Workflow завершился успешно. Его полные результаты сохранены в `artifacts/linux`:

- Miri: все 12 интеграционных тестов прошли;
- ASan: ключевой тест освобождения памяти прошёл без ошибок;
- TSan: многопоточный тест прошёл без сообщений о гонках;
- Valgrind: `ERROR SUMMARY: 0 errors`, `definitely lost: 0 bytes` для тестов и `demo`;
- Callgrind: сохранены исходный профиль, текстовый отчёт и PNG-граф вызовов.

Для локального повторения в Linux подготовлены `scripts/valgrind.sh`, `scripts/sanitize.sh` и `scripts/profile.sh`.

В исходном проекте нет FFI или C-библиотеки. Valgrind-сценарий проверяет нативный `demo` на утечки и invalid read/write.

## Windows ASan

Попытка `RUSTFLAGS=-Zsanitizer=address cargo +nightly test` выполнена. Линковщик завершился с `LNK1104`: отсутствует `clang_rt.asan_dynamic_runtime_thunk-x86_64.lib`. Поэтому ASan выполнен в Linux CI.

## Windows Performance Recorder

WPR присутствует, но запуск профиля `CPU.Verbose.File` дважды завершился кодом `0xc5585011`: системе не удалось включить право профилирования производительности. Для Windows сохранён воспроизводимый текстовый профиль с фиксированным числом вызовов, а в Linux CI получены Callgrind-отчёт и PNG-граф.
