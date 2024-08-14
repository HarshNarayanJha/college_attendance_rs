use std::collections::HashMap;

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
    pub subjects: HashMap<Subject, AttendanceEntry>,
}

impl Attendance {
    pub fn new() -> Result<Self> {
        let mut reader = csv::Reader::from_path("./data/attendance.csv")?;

        let _headers = reader.headers()?;

        let mut subjects = HashMap::new();

        for result in reader.deserialize() {
            let entry: AttendanceEntry = result?;
            subjects.insert(entry.subject, entry);
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
