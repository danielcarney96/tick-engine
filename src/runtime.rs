use crate::{GameCommand, GameEngine, GameState, TickResult};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineRequest {
    Command(GameCommand),
    Tick,
    GetState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineResponse {
    Queued,
    Tick(TickResult),
    State(GameState),
}

#[derive(Debug)]
pub struct EngineRuntime {
    engine: GameEngine,
}

impl EngineRuntime {
    pub fn new(state: GameState) -> Self {
        Self {
            engine: GameEngine::new(state),
        }
    }

    pub fn handle_request(&mut self, request: EngineRequest) -> EngineResponse {
        match request {
            EngineRequest::Command(command) => {
                self.engine.enqueue_command(command);
                EngineResponse::Queued
            }
            EngineRequest::Tick => EngineResponse::Tick(self.tick()),
            EngineRequest::GetState => EngineResponse::State(self.engine.state().clone()),
        }
    }

    pub fn tick(&mut self) -> TickResult {
        self.engine.tick()
    }

    pub fn state(&self) -> &GameState {
        self.engine.state()
    }
}
