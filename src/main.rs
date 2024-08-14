use std::{fs, path::Path, time::SystemTime};

pub mod attendance;
pub mod timetable;

use attendance::Attendance;
use chrono::{DateTime, Datelike, Local, Utc};
use timetable::{Subject, TimeTable};

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

    let mut attendance = match Attendance::new() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading attendance data -> {}", e);
            return;
        }
    };

    println!("Classes Counter in Rust\n");

    let local: DateTime<Local> = Local::now();
    // println!(
    //     "Date/Time created using SystemTime: {}",
    //     local.format("%d-%b-%Y %H:%M:%S %P %z")
    // );

    let today = local.format("%d %b %Y");
    let day = local.weekday();

    let todays_subjects = match time_table.subjects_on(local.weekday()) {
        Some(subs) => subs,
        None => {
            eprintln!("No classes today!");
            return;
        }
    };

    println!("{}", today);
    println!("Today is {}", day);
    println!("Today's subjects are {:?}", todays_subjects);

    for &sub in todays_subjects {
        attendance
            .subjects
            .entry(sub)
            .and_modify(|classes| classes.classes += 1);
    }

    println!("{:?}", attendance);

    // attendance.save();
}
