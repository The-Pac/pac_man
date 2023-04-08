use std::fs::OpenOptions;
use std::io::{Read, Write};
use log::error;
use tauri::command;
use crate::models::cell::{BlockGroup, Cell};

const MAP_HEIGHT: usize = 31;
const MAP_WIDTH: usize = 28;

#[command]
pub fn get_map() -> Vec<Vec<Cell>> {
    let mut map_file = OpenOptions::new().write(false).read(true).open("src/configs/map/map.json").expect("failed to read the map");
    let mut map_file_str = String::new();
    map_file.read_to_string(&mut map_file_str).expect("Failed to read to file");
    if map_file_str.len() > 0 {
        serde_json::from_str::<Vec<Vec<Cell>>>(map_file_str.as_str()).unwrap()
    } else {
        new_map()
    }
}

#[command]
pub fn new_map() -> Vec<Vec<Cell>> {
    vec![vec![Cell {
        orientations: vec![],
        block_group: BlockGroup::VOID,
    }; MAP_WIDTH]; MAP_HEIGHT]
}

#[command]
pub fn save_map(map_cells: String) -> Result<(), ()> {
    let mut map = OpenOptions::new().write(true).create(true).truncate(true).open("src/configs/map/map.json").expect("failed to get the map");
    match map.write_all(map_cells.as_bytes()) {
        Ok(_) => {
            Ok(())
        }
        Err(error) => {
            error!("{}",error);
            Err(())
        }
    }
}