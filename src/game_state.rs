pub type Position = (u32, u32);
pub type NpcId = u32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub player: Player,
    pub npcs: Vec<Npc>,
    pub max_move_distance_walk: u32,
    pub max_move_distance_run: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub position: Position,
    pub movement_destination: Option<Position>,
    pub health: u32,
    pub is_running: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Npc {
    pub id: NpcId,
    pub position: Position,
    pub health: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player: Player::new((0, 0)),
            npcs: Vec::new(),
            max_move_distance_walk: 1,
            max_move_distance_run: 2,
        }
    }

    pub fn add_npc(&mut self, npc: Npc) {
        self.npcs.push(npc);
    }

    pub fn remove_npc(&mut self, npc_id: u32) {
        self.npcs.retain(|npc| npc.id != npc_id);
    }
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            movement_destination: None,
            health: 99,
            is_running: false,
        }
    }
}

impl Npc {
    pub fn new(id: NpcId, position: Position, health: u32) -> Self {
        Self {
            id,
            position,
            health,
        }
    }
}
