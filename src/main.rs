use std::{fs, path::Path};

pub mod timetable;

use timetable::TimeTable;

fn main() {
    let time_table_path = Path::new("./data/timetable.json");
    let time_table_data = match fs::read_to_string(time_table_path) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error opening time table");
            return;
        }
    };

    let time_table = match TimeTable::new(&time_table_data) {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Error parsing time table -> {}", e);
            return;
        }
    };

    println!("{:#?}", time_table);
}
