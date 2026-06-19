//! Headless simulation driver. Same engine as the web server, but the clock is
//! a plain loop instead of a 600ms timer — run thousands of ticks instantly, no
//! network. This is the "backend-only simulation" half of the design: the engine
//! is a library, drivers differ only in how fast they turn the crank.

use osrs_engine::{EngineRequest, EngineRuntime, GameCommand, GameState, Npc};

fn main() {
    let mut state = GameState::new();
    state.player.health = 30;
    let mut goblin = Npc::new(1, (3, 0), 20);
    goblin.aggressive = true;
    state.add_npc(goblin);

    let mut runtime = EngineRuntime::new(state);

    // Walk to the goblin and fight it.
    for cmd in [
        GameCommand::SetAutoRetaliate { enabled: true },
        GameCommand::MovePlayer { destination: (3, 0) },
        GameCommand::Attack { target: 1 },
    ] {
        runtime.handle_request(EngineRequest::Command(cmd));
    }

    for tick in 1..=40 {
        let result = runtime.tick();
        if !result.events.is_empty() {
            println!("tick {tick:>2}: {:?}", result.events);
        }
        if runtime.state().npcs.is_empty() {
            println!(
                "goblin dead after {tick} ticks; player hp = {}",
                runtime.state().player.health
            );
            return;
        }
    }
    println!("sim ended; final state = {:?}", runtime.state());
}
