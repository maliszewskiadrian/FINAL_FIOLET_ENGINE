
# Contributing to FIOLET

## How to Contribute

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Check with clippy: `cargo clippy`
7. Commit: `git commit -m 'Add amazing feature'`
8. Push: `git push origin feature/amazing-feature`
9. Open a Pull Request

## Code Style

- Use `cargo fmt` before committing
- Ensure all tests pass
- Add tests for new features
- Document public APIs

## Reporting Bugs

Use GitHub Issues with the `bug` label.
Include:
- Rust version
- Steps to reproduce
- Expected vs actual behavior
```

---

## **📋 CHECKLIST WYKONANIA:**
```
[ ] 1. Skopiuj wszystkie pliki .rs do fiolet-core/src/
[ ] 2. Skopiuj basic_usage.rs do examples/
[ ] 3. Skopiuj integration_tests.rs do test/
[ ] 4. Skopiuj API.md do docs/
[ ] 5. Skopiuj rust.yml do .github/workflows/
[ ] 6. Skopiuj LICENSE do głównego katalogu
[ ] 7. Zaktualizuj Cargo.toml w głównym katalogu
[ ] 8. Dodaj CONTRIBUTING.md
[ ] 9. Uruchom: cd fiolet-core && cargo build
[ ] 10. Uruchom: cargo test
[ ] 11. Uruchom: cargo run --example basic_usage
[ ] 12. Git add, commit, push
