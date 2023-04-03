use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Cell {
    pub orientation: Vec<char>,
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