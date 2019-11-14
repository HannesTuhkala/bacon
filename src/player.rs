
pub struct Player {
    pub id: u8,
    pub name: &str,
    pub level: u8,
    pub health: i32,
    pub attributes: PlayerAttributes,
}

impl Player {
    pub fn new(id: u8, name: &str, level: u8) -> Self {
        Player {
            id,
            name,
            level,
            health = 500,
            attributes: PlayerAttributes::new(),
        }
    }
}

pub struct PlayerAttributes {
    pub strength: u8,
    pub agility: u8,
    pub speed: u8,
    pub intellect: u8,
    pub spirit: u8,
}

impl PlayerAttributes {
    pub fn new() -> Self {
        PlayerAttributes {
            strength: 10,
            agility: 10,
            speed: 10,
            intellect: 10,
            spirit: 10,
        }
    }
}
