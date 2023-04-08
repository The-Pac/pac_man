use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Cell {
    pub orientations: Vec<Orientation>,
    pub block_group: BlockGroup,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum BlockGroup {
    VOID,
    WALL,
    DOT,
    SUPERDOT,
    OBJECT,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Orientation {
    LEFT,
    RIGHT,
    BOTTOM,
    TOP,
}