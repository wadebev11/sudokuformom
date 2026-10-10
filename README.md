# Sudoku for Mom

A Sudoku game for one player on a phone. It runs entirely in the browser as a
Rust/WebAssembly app with no server, and it can be installed to the home screen
and played offline. See [GLOSSARY.md](GLOSSARY.md) for the game's vocabulary and
[docs/adr](docs/adr) for the design decisions.

## Layout

| Path                 | What it is                                                            |
|----------------------|-----------------------------------------------------------------------|
| `crates/sudoku-core` | Rules, solver, Techniques, grading, Hints, game state, stats          |
| `crates/puzzle-gen`  | CLI that generates the bundled puzzle bank                            |
| `crates/app`         | The Leptos web app, built with Trunk                                  |
| `e2e`                | Playwright tests that drive the app in a phone-sized browser          |

## Setup

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
(cd e2e && npm install)
```

Trunk installs to `~/.cargo/bin`, which must be on your `PATH`.

## Develop

```sh
cd crates/app && trunk serve --open   # http://127.0.0.1:8080, reloads on change
```

## Test

```sh
cargo fmt --check
cargo clippy --all-targets
cargo clippy --target wasm32-unknown-unknown -p sudokuformom
cargo test
(cd e2e && npx playwright test)
```

## Regenerate the puzzle bank

The bank is committed, so this is only needed after changing the Techniques or
the generator. It takes about 20 minutes on 6 cores, mostly finding Hard puzzles.

```sh
cargo run --release -p puzzle-gen   # writes crates/app/puzzles.bin
```

## Build and deploy

```sh
cd crates/app && trunk build --release
```

Upload everything in `crates/app/dist/`. URLs are relative, so it works from a
domain root or a subpath such as a GitHub Pages project site. The host must
serve over HTTPS for the offline support to work.

Pushes to `master` build and deploy automatically to
<https://wadebev11.github.io/sudokuformom/> via `.github/workflows/pages.yml`.
