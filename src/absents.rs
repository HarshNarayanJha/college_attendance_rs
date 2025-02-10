use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;

use crate::timetable::Subject;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Absents {
    pub absents: HashMap<Subject, Vec<String>>,
    pub extras: HashMap<Subject, Vec<String>>,
}

impl Display for Absents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nSubject\t\tAbsents").ok();
        for (sub, entry) in self.absents.iter() {
            write!(f, "{:?}", sub).ok();
            for date in entry {
                writeln!(f, "\t\t{}", date).ok();
            }
            writeln!(f, "Total {} absents", entry.len()).ok();
            writeln!(f, "").ok();
        }

        writeln!(f, "\nExtra Absents").ok();
        for (sub, entry) in self.extras.iter() {
            write!(f, "{:?}", sub).ok();
            for date in entry {
                writeln!(f, "\t\t{}", date).ok();
            }
            writeln!(f, "Total {} absents", entry.len()).ok();
            writeln!(f, "").ok();
        }

        write!(f, "")
    }
}

impl Absents {
    pub fn new() -> Self {
        let absents_path = Path::new("./data/absents.json");

        if absents_path.exists() {
            let text = std::fs::read_to_string(absents_path).unwrap();
            serde_json::from_str(&text).unwrap()
        } else {
            Self {
                absents: HashMap::new(),
                extras: HashMap::new(),
            }
        }
    }

    pub fn save(&self) -> Result<(), ()> {
        let absents_path = Path::new("./data/absents.json");
        std::fs::write(absents_path, serde_json::to_string_pretty(&self).unwrap()).unwrap();
        Ok(())
    }
}
