use csv;
use csv::Result;
use serde::{Deserialize, Serialize};

use crate::timetable::Subject;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AttendanceEntry {
    subject: Subject,
    classes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attendance {
    subjects: Vec<AttendanceEntry>,
}

impl Attendance {
    pub fn new() -> Result<Self> {
        let mut reader = csv::Reader::from_path("./data/attendance.csv")?;

        let headers = reader.headers()?;
        println!("{:?}", headers);

        let mut subjects = Vec::new();

        for result in reader.deserialize() {
            let entry: AttendanceEntry = result?;
            subjects.push(entry);
        }

        Ok(Self { subjects })
    }
}
