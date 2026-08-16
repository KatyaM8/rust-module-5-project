# Linux-проверки

Эта папка сформирована реальным запуском `.github/workflows/verification.yml` на Ubuntu и автоматически сохранена в репозитории после успешного завершения всех задач.

- `tests-and-miri` — `cargo check`, `cargo test`, сборка workspace и Miri;
- `sanitizers` — отдельные журналы AddressSanitizer и ThreadSanitizer;
- `valgrind-and-profile` — Valgrind для тестов и `demo`, Callgrind-профиль, текстовый отчёт и PNG-граф вызовов.

Краткий итог: 12 интеграционных тестов прошли под Miri, тесты ASan/TSan прошли, Valgrind не обнаружил ошибок памяти и definitely lost утечек.
