# Contributing to todo

Thanks for considering a contribution — bug reports, feature ideas, docs fixes, and pull requests are all welcome.

## Reporting a bug

Open an [issue](https://github.com/RayanBO/todo/issues/new) and include:

- Your OS (Windows / macOS / Linux) and `todo --version`
- The command you ran and what you expected to happen
- What actually happened (error message, screenshot, or a snippet of your `TODO.md` if relevant)

## Suggesting a feature

Open an issue describing the use case first, before writing code — this avoids wasted work if the idea doesn't fit the project's scope (single binary, zero dependencies, flat-file storage).

## Development setup

Requires [Rust](https://rustup.rs/) 1.75+.

```bash
git clone https://github.com/RayanBO/todo.git
cd todo
cargo build --release
cargo test
```

The binary is built at `target/release/todo` (or `todo.exe` on Windows).

## Project structure

```
src/
  main.rs        # CLI entry point, argument parsing
  commands.rs    # command implementations (add, list, update, delete, status...)
  dashboard.rs   # embedded HTTP server + web dashboard
  models.rs      # Task / Actor / Comment data structures
  parser.rs      # TODO.md parsing
  serializer.rs  # TODO.md writing
  id_gen.rs      # 4-char ID generation
  tags.rs        # tag handling
tests/
  integration_test.rs
dashboard/
  index.html     # dashboard static SPA
```

## Making a pull request

1. Fork the repo and create a branch from `main`
2. Keep the change focused — one feature or fix per PR
3. Run `cargo test` and make sure it passes
4. Run `cargo fmt` before committing
5. Update `README.md` / `CHANGELOG.md` / `skills/SKILL.md` if your change affects CLI commands, flags, or the API
6. Open the PR with a short description of what changed and why

## Coding style

- Standard `rustfmt` formatting (`cargo fmt`)
- Keep dependencies to a minimum — this project is intentionally zero-dependency-leaning; new crates need a good reason
- Prefer clear, explicit code over clever abstractions

## Code of conduct

Be respectful and constructive. Disagreements about design are fine; personal attacks aren't.

## License

By contributing, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
