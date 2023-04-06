use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Cell {
    pub orientations: Vec<Orientation>,
    pub x: i32,
    pub y: i32,
    pub block_group: BlockGroup,
}

#[derive(Deserialize, Serialize)]
pub enum BlockGroup {
    VOID,
    WALL,
    DOT,
    SUPERDOT,
    OBJECT,
}

#[derive(Deserialize, Serialize)]
pub enum Orientation {
    LEFT,
    RIGHT,
    BOTTOM,
    TOP,
}