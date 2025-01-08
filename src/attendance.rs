use std::{
    fmt::Display,
    fs::{self, File},
    path::Path,
    process::exit,
};

use csv;
use csv::Result;
use serde::{Deserialize, Serialize};

use crate::timetable::Subject;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttendanceEntry {
    pub subject: Subject,
    pub classes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attendance {
    pub subjects: Vec<(Subject, AttendanceEntry)>,
}

impl Display for Attendance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nSubject\t\tClasses").ok();
        for (sub, entry) in self.subjects.iter() {
            writeln!(f, "{:?}\t\t{}", sub, entry.classes).ok();
        }

        write!(f, "")
    }
}

impl Attendance {
    pub fn new() -> Result<Self> {
        let attendance_path = Path::new("./data/attendance.csv");
        match attendance_path.try_exists() {
            Ok(false) => {
                println!("Attendance File doesn't exists, trying to create a new one...");
                if let Err(e) = File::create(attendance_path) {
                    eprintln!("Failed to create timetable file: {}", e);
                    return Err(e.into());
                }
                println!("Created new attendance file");
                println!(
                    "Please type out the subject names manually, then run this program again!"
                );
                exit(0);
            }
            Ok(true) => {
                if fs::read_to_string(attendance_path)
                    .unwrap()
                    .trim_ascii()
                    .is_empty()
                {
                    println!("I asked you to type all the subject names manually... please do so and then only run the program");
                    exit(1);
                }
            }
            Err(_) => {
                eprintln!("Some serious issue is going on with you filesystem, run the project somewhere else...");
                exit(1);
            }
        }

        let mut reader = csv::Reader::from_path(attendance_path)?;

        let _headers = reader.headers()?;

        let mut subjects = Vec::new();

        for result in reader.deserialize() {
            let entry: AttendanceEntry = result?;
            subjects.push((entry.subject, entry));
        }

        Ok(Self { subjects })
    }

    pub fn save(&mut self) -> Result<()> {
        let mut writer = csv::Writer::from_path("./data/attendance.csv")?;

        for (_, entry) in self.subjects.iter() {
            writer.serialize(entry)?;
        }

        Ok(())
    }
}
