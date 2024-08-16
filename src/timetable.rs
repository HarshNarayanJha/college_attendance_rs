use std::fmt::Display;

use chrono::Weekday;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Deserialize, Serialize, Eq, Hash, PartialEq, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Subject {
    OOP,
    DSD,
    DM,
    DSLAB,
    COA,
    DS,
    NSSPT,
    BEELAB,
    OOPLAB,
    DSDLAB,
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::OOP => write!(f, "OOP"),
            Subject::DSD => write!(f, "DSD"),
            Subject::DM => write!(f, "DM"),
            Subject::DSLAB => write!(f, "DSLAB"),
            Subject::COA => write!(f, "COA"),
            Subject::DS => write!(f, "DS"),
            Subject::NSSPT => write!(f, "NSSPT"),
            Subject::BEELAB => write!(f, "BEELAB"),
            Subject::OOPLAB => write!(f, "OOPLAB"),
            Subject::DSDLAB => write!(f, "DSDLAB"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct TimeTable {
    mon: Vec<Subject>,
    tue: Vec<Subject>,
    wed: Vec<Subject>,
    thu: Vec<Subject>,
    fri: Vec<Subject>,
}

impl TimeTable {
    pub fn new(json_data: &str) -> Result<Self> {
        match serde_json::from_str(json_data) {
            Ok(data) => Ok(data),
            Err(e) => Err(e)
        }
    }

    pub fn subjects_on(&self, weekday: Weekday) -> Option<&Vec<Subject>> {
        match weekday {
            Weekday::Mon => Some(&self.mon),
            Weekday::Tue => Some(&self.tue),
            Weekday::Wed => Some(&self.wed),
            Weekday::Thu => Some(&self.thu),
            Weekday::Fri => Some(&self.fri),
            _ => None
        }
    }
}
