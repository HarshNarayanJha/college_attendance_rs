use std::fmt::Display;

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
        let mut reader = csv::Reader::from_path("./data/attendance.csv")?;

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
