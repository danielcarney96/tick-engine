use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::game_state::{GameState, NpcId, Position};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameCommand {
    MovePlayer { destination: Position },
    SetRun { enabled: bool },
    Attack { target: NpcId },
    SetAutoRetaliate { enabled: bool },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    PlayerRunChanged {
        enabled: bool,
    },
    PlayerMoved {
        from: Position,
        to: Position,
    },
    AutoRetaliateChanged {
        enabled: bool,
    },
    PlayerAttacked {
        target: NpcId,
        damage: u32,
    },
    NpcAttacked {
        attacker: NpcId,
        damage: u32,
    },
    NpcDied {
        id: NpcId,
    },
    PlayerDied,
    CommandRejected {
        command: GameCommand,
        reason: CommandRejectionReason,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandRejectionReason {
    AlreadyAtDestination,
    NoMovementAvailable,
    TargetNotFound,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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

        self.update_combat_pathing();
        self.advance_player_movement(&mut result);
        self.resolve_combat(&mut result);

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
            GameCommand::Attack { target } => {
                if self.state.npcs.iter().any(|npc| npc.id == target) {
                    self.state.player.target = Some(target);
                } else {
                    result.events.push(GameEvent::CommandRejected {
                        command: GameCommand::Attack { target },
                        reason: CommandRejectionReason::TargetNotFound,
                    });
                }
            }
            GameCommand::SetAutoRetaliate { enabled } => {
                if self.state.player.auto_retaliate != enabled {
                    self.state.player.auto_retaliate = enabled;
                    result
                        .events
                        .push(GameEvent::AutoRetaliateChanged { enabled });
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

    /// Walk the player toward its combat target until within attack range, then stop.
    fn update_combat_pathing(&mut self) {
        let Some(target) = self.state.player.target else {
            return;
        };
        let Some(npc) = self.state.npcs.iter().find(|npc| npc.id == target) else {
            return;
        };

        if in_range(self.state.player.position, npc.position, self.state.attack_range) {
            self.state.player.movement_destination = None;
        } else {
            // stop-at-adjacent pathing when collision/positioning matters.
            self.state.player.movement_destination = Some(npc.position);
        }
    }

    fn resolve_combat(&mut self, result: &mut TickResult) {
        self.state.player.attack_cooldown = self.state.player.attack_cooldown.saturating_sub(1);
        for npc in &mut self.state.npcs {
            npc.attack_cooldown = npc.attack_cooldown.saturating_sub(1);
        }

        self.resolve_player_attack(result);
        self.resolve_npc_attacks(result);
    }

    fn resolve_player_attack(&mut self, result: &mut TickResult) {
        let Some(target) = self.state.player.target else {
            return;
        };
        let Some(idx) = self.state.npcs.iter().position(|npc| npc.id == target) else {
            self.state.player.target = None;
            return;
        };

        let npc_pos = self.state.npcs[idx].position;
        if !in_range(self.state.player.position, npc_pos, self.state.attack_range)
            || self.state.player.attack_cooldown > 0
        {
            return;
        }

        let damage = self.state.player_attack_damage;
        let npc = &mut self.state.npcs[idx];
        npc.in_combat = true;
        npc.health = npc.health.saturating_sub(damage);
        self.state.player.attack_cooldown = self.state.attack_speed;
        result.events.push(GameEvent::PlayerAttacked { target, damage });

        if self.state.npcs[idx].health == 0 {
            self.state.npcs.remove(idx);
            self.state.player.target = None;
            result.events.push(GameEvent::NpcDied { id: target });
        }
    }

    fn resolve_npc_attacks(&mut self, result: &mut TickResult) {
        let player_pos = self.state.player.position;
        let range = self.state.attack_range;
        let damage = self.state.npc_attack_damage;
        let attack_speed = self.state.attack_speed;

        for i in 0..self.state.npcs.len() {
            if self.state.player.health == 0 {
                break;
            }

            let npc = &self.state.npcs[i];
            let engaged = npc.aggressive || npc.in_combat;
            if !engaged
                || npc.attack_cooldown > 0
                || !in_range(npc.position, player_pos, range)
            {
                continue;
            }

            let attacker = npc.id;
            self.state.npcs[i].in_combat = true;
            self.state.npcs[i].attack_cooldown = attack_speed;
            self.state.player.health = self.state.player.health.saturating_sub(damage);
            result.events.push(GameEvent::NpcAttacked { attacker, damage });

            if self.state.player.auto_retaliate && self.state.player.target.is_none() {
                self.state.player.target = Some(attacker);
            }

            if self.state.player.health == 0 {
                result.events.push(GameEvent::PlayerDied);
            }
        }
    }
}

/// Chebyshev distance: how OSRS measures melee/attack range on the tile grid.
fn in_range(a: Position, b: Position, range: u32) -> bool {
    a.0.abs_diff(b.0).max(a.1.abs_diff(b.1)) <= range
}

fn step_toward(from: Position, destination: Position, max_distance: u32) -> Position {
    // Chebyshev movement (matches `in_range`): one tile of movement can advance
    // both axes at once, so a diagonal step costs the same as a straight one.
    let (mut x, mut y) = from;
    for _ in 0..max_distance {
        if (x, y) == destination {
            break;
        }
        x += (x < destination.0) as u32;
        x -= (x > destination.0) as u32;
        y += (y < destination.1) as u32;
        y -= (y > destination.1) as u32;
    }
    (x, y)
}
