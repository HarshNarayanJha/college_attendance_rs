use std::fmt::Display;

use chrono::Weekday;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Deserialize, Serialize, Eq, Hash, PartialEq, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Subject {
    NM,
    DBMS,
    DAA,
    OS,
    UHV,
    NSSPT,
    DBMSLAB,
    NMLAB,
    SHELL,

    // SEM 5
    DCCN,
    BITW,
    DCCNLAB,
    SP,
    OT,
    SE,
    SELAB,
    FLAT,
    DMW,
    DMWLAB,
    SC,
}

impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Subject::NM => write!(f, "NM"),
            Subject::DBMS => write!(f, "DBMS"),
            Subject::DAA => write!(f, "DAA"),
            Subject::OS => write!(f, "OS"),
            Subject::UHV => write!(f, "UHV"),
            Subject::NSSPT => write!(f, "NSSPT"),
            Subject::DBMSLAB => write!(f, "DBMSLAB"),
            Subject::NMLAB => write!(f, "NMLAB"),
            Subject::SHELL => write!(f, "SHELL"),

            // SEM 5
            Subject::DCCN => write!(f, "DCCN"),
            Subject::BITW => write!(f, "BITW"),
            Subject::DCCNLAB => write!(f, "DCCNLAB"),
            Subject::SP => write!(f, "SP"),
            Subject::OT => write!(f, "OT"),
            Subject::SE => write!(f, "SE"),
            Subject::SELAB => write!(f, "SELAB"),
            Subject::FLAT => write!(f, "FLAT"),
            Subject::DMW => write!(f, "DMW"),
            Subject::DMWLAB => write!(f, "DMWLAB"),
            Subject::SC => write!(f, "SC"),
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
            Err(e) => Err(e),
        }
    }

    pub fn subjects_on(&self, weekday: Weekday) -> Option<&Vec<Subject>> {
        match weekday {
            Weekday::Mon => Some(&self.mon),
            Weekday::Tue => Some(&self.tue),
            Weekday::Wed => Some(&self.wed),
            Weekday::Thu => Some(&self.thu),
            Weekday::Fri => Some(&self.fri),
            _ => None,
        }
    }
}
