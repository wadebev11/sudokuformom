# Client-only WASM app, no backend

Mom plays on her phone, and nothing needs to live on a server, so the whole game (solver, Hints, game state, stats) runs in the browser as a Rust/WASM PWA built with Leptos (client-side rendering) and Trunk. It is deployed as a static `dist/` folder. This replaces the original `tide` server scaffold.

## Considered Options

- **Axum + Leptos SSR backend**: rejected. It is only worth it for syncing across devices or tracking progress remotely, and neither is wanted.

## Consequences

- Puzzles are pre-generated at build time by a CLI and compiled into the `.wasm`, because generating graded Hard and Expert puzzles on a phone is too slow to do in the moment.
- All persistence (game in progress, settings, stats) is local to her phone's browser. Clearing site data or switching phones loses it.
