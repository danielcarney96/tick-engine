use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use osrs_engine::{GameCommand, GameEngine, GameState, Npc};

fn main() {
    let tick_rate = Duration::from_millis(600);
    let mut next_tick = Instant::now();

    let mut game_state = GameState::new();
    game_state.add_npc(Npc::new(1, (5, 0), 10));
    let mut engine = GameEngine::new(game_state);
    engine.enqueue_command(GameCommand::MovePlayer {
        destination: (10, 0),
    });

    loop {
        game_loop(&mut engine);

        next_tick += tick_rate;

        let now = Instant::now();
        if now < next_tick {
            sleep(next_tick - now);
        } else {
            eprintln!("tick lagging behind by {:?}", now - next_tick);
            next_tick = now;
        }
    }
}

fn game_loop(engine: &mut GameEngine) {
    println!("tick");

    let result = engine.tick();

    println!("events: {:?}", result.events);
    println!("state: {:?}", engine.state());

    // Movement
    // If requested position, clear movement queue
    // Move min(requested_position, max_move_distance_walk) if walking
    // Move min(requested_position, max_move_distance_run) if running
    // Queue up remaining movement for next tick if needed
    // Collect and validate input
    // Movement
    // Interactions
    // Attacking
    // Prayers
    // Item interacting (eating, equipping, fletching, etc)
    // Begin attack
    // Check valid
    // Check LOS and distance
    // Move to target if needed
    // Roll accuracy
    // Roll damage
    // Attack happens
    // Wait for weapon delay
    // Deal damange
    // Add to weapon delay
    // Death (player and npc)
    // NPC behaviour
    // Send events through interface
}
