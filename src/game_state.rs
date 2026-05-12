#[derive(Debug)]
pub struct GameState {
    pub player: Player,
    pub npcs: Vec<Npc>,
    pub max_move_distance_walk: u32,
    pub max_move_distance_run: u32,
}

#[derive(Debug)]
pub struct Player {
    pub position: (u32, u32),
    pub health: u32,
    pub is_running: bool,
}

#[derive(Debug)]
pub struct Npc {
    pub id: u32,
    pub position: (u32, u32),
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
    pub fn new(position: (u32, u32)) -> Self {
        Self {
            position,
            health: 99,
            is_running: false,
        }
    }
}

impl Npc {
    pub fn new(id: u32, position: (u32, u32), health: u32) -> Self {
        Self {
            id,
            position,
            health,
        }
    }
}
