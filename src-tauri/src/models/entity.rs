pub struct Entity {
    pub name: Name,
    pub state: State,
    pub is_ai: bool,
}

pub enum State {
    CHASE,
    SCATTER,
    FRIGHTENED,
    EATEN,
}

pub enum Name {
    BLINKY,
    PINKY,
    INKY,
    CLYDE,
    PACMAN,
}

