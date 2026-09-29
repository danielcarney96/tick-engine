# osrs-engine

A tick-based game engine in the style of Old School RuneScape, written in Rust. The engine is a transport-agnostic library, and several binaries drive it at different speeds.

Current mechanics: tile movement (walk and run), melee combat, NPC aggression and auto-retaliate.

## Running

| Command | What it does |
|---|---|
| `cargo run --bin web` | WebSocket server and browser UI at http://127.0.0.1:8080, ticking every 600ms |
| `cargo run --bin sim` | Headless simulation that runs ticks in a plain loop with no network |
| `cargo run` | Line-based TCP server on `127.0.0.1:43594` (you can pass an address as the first argument) |
| `cargo run --bin cli [addr]` | Interactive client for the TCP server |

CLI commands: `move <x> <y>`, `run <on\|off>`, `attack <npc_id>`, `retaliate <on\|off>`, `tick`, `state`, `help`, `quit`.

## Layout

- `src/game_state.rs`: `GameState`, `Player`, `Npc`, `Position`
- `src/engine.rs`: `GameEngine`, which queues `GameCommand`s and emits `GameEvent`s on each `tick()`
- `src/runtime.rs`: `EngineRuntime`, the request/response wrapper that the drivers use
- `src/server.rs`: TCP server (the `osrs-engine` binary)
- `src/bin/`: the `web`, `sim` and `cli` drivers
- `web/index.html`: browser frontend

## Tests

```sh
cargo test
```
