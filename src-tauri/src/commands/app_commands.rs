use std::fs::OpenOptions;
use std::io::Read;
use crate::models::cell::Cell;

pub fn get_map(){
    let mut map_file = OpenOptions::new().write(false).read(true).open("src/configs/map/map.json").expect("failed to read the map");
    let mut map_file_str = String::new();
    map_file.read_to_string(&mut map_file_str).expect("Failed to read to file");
    let map = serde_json::from_str::<Vec<Cell>>(map_file_str.as_str()).unwrap();
}