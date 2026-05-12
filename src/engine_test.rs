use crate::{CommandRejectionReason, GameCommand, GameEngine, GameEvent, GameState, Npc, Player};

#[test]
fn callers_can_construct_arbitrary_state() {
    let state = GameState {
        player: Player {
            position: (10, 10),
            movement_destination: Some((12, 10)),
            health: 42,
            is_running: true,
        },
        npcs: vec![Npc::new(1, (5, 5), 100), Npc::new(2, (8, 8), 250)],
        max_move_distance_walk: 1,
        max_move_distance_run: 2,
    };

    assert_eq!(state.player.position, (10, 10));
    assert_eq!(state.player.movement_destination, Some((12, 10)));
    assert_eq!(state.player.health, 42);
    assert_eq!(state.npcs.len(), 2);
}

#[test]
fn engine_preserves_provided_state() {
    let mut state = GameState::new();
    state.player = Player::new((4, 3));
    state.add_npc(Npc::new(7, (9, 9), 12));

    let engine = GameEngine::new(state.clone());

    assert_eq!(engine.state(), &state);
}

#[test]
fn set_run_toggles_player_running_and_emits_event() {
    let mut engine = GameEngine::new(GameState::new());

    engine.enqueue_command(GameCommand::SetRun { enabled: true });
    let result = engine.tick();

    assert!(engine.state().player.is_running);
    assert_eq!(
        result.events,
        vec![GameEvent::PlayerRunChanged { enabled: true }]
    );
}

#[test]
fn move_player_walks_one_tile_when_not_running() {
    let mut engine = GameEngine::new(GameState::new());

    engine.enqueue_command(GameCommand::MovePlayer {
        destination: (10, 0),
    });
    let result = engine.tick();

    assert_eq!(engine.state().player.position, (1, 0));
    assert_eq!(
        result.events,
        vec![GameEvent::PlayerMoved {
            from: (0, 0),
            to: (1, 0)
        }]
    );
    assert_eq!(engine.state().player.movement_destination, Some((10, 0)));
}

#[test]
fn move_player_runs_two_tiles_when_running() {
    let mut state = GameState::new();
    state.player.is_running = true;
    let mut engine = GameEngine::new(state);

    engine.enqueue_command(GameCommand::MovePlayer {
        destination: (10, 0),
    });
    let result = engine.tick();

    assert_eq!(engine.state().player.position, (2, 0));
    assert_eq!(
        result.events,
        vec![GameEvent::PlayerMoved {
            from: (0, 0),
            to: (2, 0)
        }]
    );
    assert_eq!(engine.state().player.movement_destination, Some((10, 0)));
}

#[test]
fn single_move_command_continues_across_ticks_until_destination_is_reached() {
    let mut engine = GameEngine::new(GameState::new());

    engine.enqueue_command(GameCommand::MovePlayer {
        destination: (3, 0),
    });

    let first_tick = engine.tick();
    let second_tick = engine.tick();
    let third_tick = engine.tick();
    let fourth_tick = engine.tick();

    assert_eq!(engine.state().player.position, (3, 0));
    assert_eq!(engine.state().player.movement_destination, None);
    assert_eq!(
        first_tick.events,
        vec![GameEvent::PlayerMoved {
            from: (0, 0),
            to: (1, 0)
        }]
    );
    assert_eq!(
        second_tick.events,
        vec![GameEvent::PlayerMoved {
            from: (1, 0),
            to: (2, 0)
        }]
    );
    assert_eq!(
        third_tick.events,
        vec![GameEvent::PlayerMoved {
            from: (2, 0),
            to: (3, 0)
        }]
    );
    assert!(fourth_tick.events.is_empty());
}

#[test]
fn no_op_movement_is_rejected_without_mutating_position() {
    let mut engine = GameEngine::new(GameState::new());

    engine.enqueue_command(GameCommand::MovePlayer {
        destination: (0, 0),
    });
    let result = engine.tick();

    assert_eq!(engine.state().player.position, (0, 0));
    assert_eq!(engine.state().player.movement_destination, None);
    assert_eq!(
        result.events,
        vec![GameEvent::CommandRejected {
            command: GameCommand::MovePlayer {
                destination: (0, 0)
            },
            reason: CommandRejectionReason::AlreadyAtDestination
        }]
    );
}

#[test]
fn zero_movement_distance_keeps_destination_for_future_ticks() {
    let mut state = GameState::new();
    state.max_move_distance_walk = 0;
    let mut engine = GameEngine::new(state);

    engine.enqueue_command(GameCommand::MovePlayer {
        destination: (10, 0),
    });
    let result = engine.tick();

    assert_eq!(engine.state().player.position, (0, 0));
    assert_eq!(engine.state().player.movement_destination, Some((10, 0)));
    assert_eq!(
        result.events,
        vec![GameEvent::CommandRejected {
            command: GameCommand::MovePlayer {
                destination: (10, 0)
            },
            reason: CommandRejectionReason::NoMovementAvailable
        }]
    );
}

#[test]
fn queued_commands_are_processed_fifo_before_movement_advances() {
    let mut engine = GameEngine::new(GameState::new());

    engine.enqueue_command(GameCommand::MovePlayer {
        destination: (10, 0),
    });
    engine.enqueue_command(GameCommand::SetRun { enabled: true });
    engine.enqueue_command(GameCommand::MovePlayer {
        destination: (0, 10),
    });

    let result = engine.tick();

    assert_eq!(engine.state().player.position, (0, 2));
    assert!(engine.state().player.is_running);
    assert_eq!(engine.state().player.movement_destination, Some((0, 10)));
    assert_eq!(
        result.events,
        vec![
            GameEvent::PlayerRunChanged { enabled: true },
            GameEvent::PlayerMoved {
                from: (0, 0),
                to: (0, 2)
            },
        ]
    );
}
