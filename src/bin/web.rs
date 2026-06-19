//! WebSocket server that hosts a single live game world and serves the browser
//! frontend. The engine itself stays transport-agnostic (see the `osrs_engine`
//! lib crate); this binary is just a driver that wraps `tick()` in a wall-clock
//! timer and broadcasts state to connected browsers.
//!
//! ponytail: single shared world, one engine instance. Single-player by design —
//! every connected tab drives the same character. Per-session worlds would be the
//! upgrade path if multiplayer were ever wanted.

use std::time::Duration;

use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::{Html, Response},
    routing::get,
};
use osrs_engine::{EngineRequest, EngineRuntime, GameCommand, GameEvent, GameState, Npc};
use serde::Serialize;
use tokio::sync::{mpsc, watch};

const TICK_MS: u64 = 600; // one OSRS game tick
const ADDR: &str = "127.0.0.1:8080";

/// What the server pushes to every client each tick: the full authoritative
/// state plus the events that just happened (for animations / damage splats).
#[derive(Clone, Serialize)]
struct Snapshot {
    state: GameState,
    events: Vec<GameEvent>,
}

#[derive(Clone)]
struct AppState {
    commands: mpsc::Sender<GameCommand>,
    snapshots: watch::Receiver<Snapshot>,
}

#[tokio::main]
async fn main() {
    let state = seed_world();
    let mut runtime = EngineRuntime::new(state.clone());

    let (cmd_tx, mut cmd_rx) = mpsc::channel::<GameCommand>(64);
    let (snap_tx, snap_rx) = watch::channel(Snapshot { state, events: Vec::new() });

    // The engine task: the only thing that touches the engine. It owns the tick
    // clock and applies incoming commands. Commands enqueue immediately and take
    // effect on the next tick — server-authoritative, exactly like the headless sim.
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_millis(TICK_MS));
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    let result = runtime.tick();
                    let _ = snap_tx.send(Snapshot {
                        state: runtime.state().clone(),
                        events: result.events,
                    });
                }
                Some(cmd) = cmd_rx.recv() => {
                    runtime.handle_request(EngineRequest::Command(cmd));
                }
            }
        }
    });

    let app = Router::new()
        .route("/", get(|| async { Html(include_str!("../../web/index.html")) }))
        .route("/ws", get(ws_handler))
        .with_state(AppState { commands: cmd_tx, snapshots: snap_rx });

    println!("osrs-engine web client: http://{ADDR}");
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn seed_world() -> GameState {
    let mut state = GameState::new();
    state.player.health = 30;
    state.add_npc(Npc::new(1, (5, 3), 20));
    let mut goblin = Npc::new(2, (8, 6), 15);
    goblin.aggressive = true;
    state.add_npc(goblin);
    state
}

async fn ws_handler(ws: WebSocketUpgrade, State(app): State<AppState>) -> Response {
    ws.on_upgrade(|socket| client(socket, app))
}

async fn client(mut socket: WebSocket, app: AppState) {
    let mut snapshots = app.snapshots.clone();

    // Send the current world immediately so a fresh tab renders without waiting
    // for the next tick. watch always holds the latest value.
    let initial = snapshots.borrow_and_update().clone();
    if send_snapshot(&mut socket, &initial).await.is_err() {
        return;
    }

    loop {
        tokio::select! {
            changed = snapshots.changed() => {
                if changed.is_err() {
                    break; // engine task gone
                }
                let snap = snapshots.borrow_and_update().clone();
                if send_snapshot(&mut socket, &snap).await.is_err() {
                    break;
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(txt))) => {
                        // Clients send intents (GameCommand) only, never state.
                        if let Ok(cmd) = serde_json::from_str::<GameCommand>(&txt) {
                            let _ = app.commands.send(cmd).await;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    _ => {}
                }
            }
        }
    }
}

async fn send_snapshot(socket: &mut WebSocket, snap: &Snapshot) -> Result<(), ()> {
    let txt = serde_json::to_string(snap).map_err(|_| ())?;
    socket.send(Message::Text(txt)).await.map_err(|_| ())
}
