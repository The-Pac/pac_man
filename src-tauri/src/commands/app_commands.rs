use std::fs::OpenOptions;
use std::io::Read;
use tauri::command;
use crate::models::cell::{BlockGroup, Cell};

const MAP_HEIGHT: i32 = 31;
const MAP_WIDTH: i32 = 28;

#[command]
pub fn get_map() {
    let mut map_file = OpenOptions::new().write(false).read(true).open("src/configs/map/map.json").expect("failed to read the map");
    let mut map_file_str = String::new();
    map_file.read_to_string(&mut map_file_str).expect("Failed to read to file");
    let map = serde_json::from_str::<Vec<Cell>>(map_file_str.as_str()).unwrap();
}

#[command]
pub fn new_map() -> Vec<Vec<Cell>> {
    let mut map: Vec<Vec<Cell>> = vec![];

    for i in 0..MAP_HEIGHT {
        let mut row = vec![];
        for j in 0..MAP_WIDTH {
            row.push(Cell {
                orientation: vec![],
                x: 0,
                y: 0,
                block_group: BlockGroup::VOID,
            })
        }
        map.push(row);
    }
    map
}