use std::collections::VecDeque;

use crate::game_state::{GameState, Position};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameCommand {
    MovePlayer { destination: Position },
    SetRun { enabled: bool },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameEvent {
    PlayerRunChanged {
        enabled: bool,
    },
    PlayerMoved {
        from: Position,
        to: Position,
    },
    CommandRejected {
        command: GameCommand,
        reason: CommandRejectionReason,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandRejectionReason {
    AlreadyAtDestination,
    NoMovementAvailable,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TickResult {
    pub events: Vec<GameEvent>,
}

#[derive(Debug)]
pub struct GameEngine {
    state: GameState,
    commands: VecDeque<GameCommand>,
}

impl GameEngine {
    pub fn new(state: GameState) -> Self {
        Self {
            state,
            commands: VecDeque::new(),
        }
    }

    pub fn enqueue_command(&mut self, command: GameCommand) {
        self.commands.push_back(command);
    }

    pub fn tick(&mut self) -> TickResult {
        let mut result = TickResult::default();

        while let Some(command) = self.commands.pop_front() {
            self.apply_command(command, &mut result);
        }

        self.advance_player_movement(&mut result);

        result
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    fn apply_command(&mut self, command: GameCommand, result: &mut TickResult) {
        match command {
            GameCommand::MovePlayer { destination } => {
                self.set_player_movement_destination(
                    destination,
                    GameCommand::MovePlayer { destination },
                    result,
                );
            }
            GameCommand::SetRun { enabled } => {
                if self.state.player.is_running != enabled {
                    self.state.player.is_running = enabled;
                    result.events.push(GameEvent::PlayerRunChanged { enabled });
                }
            }
        }
    }

    fn set_player_movement_destination(
        &mut self,
        destination: Position,
        original_command: GameCommand,
        result: &mut TickResult,
    ) {
        let from = self.state.player.position;

        if from == destination {
            result.events.push(GameEvent::CommandRejected {
                command: original_command,
                reason: CommandRejectionReason::AlreadyAtDestination,
            });
            return;
        }

        self.state.player.movement_destination = Some(destination);
    }

    fn advance_player_movement(&mut self, result: &mut TickResult) {
        let Some(destination) = self.state.player.movement_destination else {
            return;
        };

        let from = self.state.player.position;
        let max_distance = if self.state.player.is_running {
            self.state.max_move_distance_run
        } else {
            self.state.max_move_distance_walk
        };
        if max_distance == 0 {
            result.events.push(GameEvent::CommandRejected {
                command: GameCommand::MovePlayer { destination },
                reason: CommandRejectionReason::NoMovementAvailable,
            });
            return;
        }

        let to = step_toward(from, destination, max_distance);
        self.state.player.position = to;
        if to == destination {
            self.state.player.movement_destination = None;
        }
        result.events.push(GameEvent::PlayerMoved { from, to });
    }
}

fn step_toward(from: Position, destination: Position, max_distance: u32) -> Position {
    let mut remaining = max_distance;
    let mut x = from.0;
    let mut y = from.1;

    let x_step = remaining.min(x.abs_diff(destination.0));
    if x < destination.0 {
        x += x_step;
    } else {
        x -= x_step;
    }
    remaining -= x_step;

    let y_step = remaining.min(y.abs_diff(destination.1));
    if y < destination.1 {
        y += y_step;
    } else {
        y -= y_step;
    }

    (x, y)
}
