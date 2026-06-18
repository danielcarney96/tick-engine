use crate::{EngineRequest, EngineResponse, EngineRuntime, GameCommand, GameEvent, GameState};

#[test]
fn command_requests_are_queued_until_tick() {
    let mut runtime = EngineRuntime::new(GameState::new());

    let response = runtime.handle_request(EngineRequest::Command(GameCommand::MovePlayer {
        destination: (3, 0),
    }));

    assert_eq!(response, EngineResponse::Queued);
    assert_eq!(runtime.state().player.position, (0, 0));
    assert_eq!(runtime.state().player.movement_destination, None);
}

#[test]
fn tick_requests_advance_the_engine() {
    let mut runtime = EngineRuntime::new(GameState::new());
    runtime.handle_request(EngineRequest::Command(GameCommand::MovePlayer {
        destination: (3, 0),
    }));

    let response = runtime.handle_request(EngineRequest::Tick);

    assert_eq!(
        response,
        EngineResponse::Tick(crate::TickResult {
            events: vec![GameEvent::PlayerMoved {
                from: (0, 0),
                to: (1, 0)
            }]
        })
    );
    assert_eq!(runtime.state().player.position, (1, 0));
}

#[test]
fn get_state_returns_a_snapshot() {
    let mut runtime = EngineRuntime::new(GameState::new());

    let response = runtime.handle_request(EngineRequest::GetState);

    assert_eq!(response, EngineResponse::State(GameState::new()));
}
