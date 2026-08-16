# Ограничения инструментов

Результаты ниже не симулировались.

## Valgrind, perf, flamegraph и TSan

`wsl.exe --list --verbose` сообщил, что WSL не установлен. Нативных Windows-версий `valgrind`, `perf` и `cargo-flamegraph` в системе нет. ThreadSanitizer для требуемого Linux-сценария поэтому также не запускался. Для повторения после установки WSL подготовлены `scripts/valgrind.sh`, `scripts/sanitize.sh` и `scripts/profile.sh`.

В исходном проекте нет FFI или C-библиотеки. Valgrind-сценарий проверяет нативный `demo` на утечки и invalid read/write.

## Windows ASan

Попытка `RUSTFLAGS=-Zsanitizer=address cargo +nightly test` выполнена. Линковщик завершился с `LNK1104`: отсутствует `clang_rt.asan_dynamic_runtime_thunk-x86_64.lib`. Компонент LLVM/ASan для установленного Visual Studio отсутствует, поэтому отчёт ASan получить нельзя.

## Windows Performance Recorder

WPR присутствует, но запуск профиля `CPU.Verbose.File` дважды завершился кодом `0xc5585011`: системе не удалось включить право профилирования производительности. Вместо отсутствующего ETL/flamegraph сохранён воспроизводимый текстовый профиль с фиксированным числом вызовов.
