use std::{collections::HashMap, fmt::Display, path::Path};

use serde::{Deserialize, Serialize};

use crate::timetable::Subject;

#[derive(Debug, Serialize, Deserialize)]
pub struct Classes {
    pub classes: HashMap<Subject, Vec<String>>,
    pub extras: HashMap<Subject, Vec<String>>,
}

impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nSubject\t\tClasses").ok();
        for (sub, entry) in self.classes.iter() {
            write!(f, "{:?}", sub).ok();
            for date in entry {
                writeln!(f, "\t\t{}", date).ok();
            }
            writeln!(f, "").ok();
        }

        writeln!(f, "\nExtra Classes").ok();
        for (sub, entry) in self.extras.iter() {
            write!(f, "{:?}", sub).ok();
            for date in entry {
                writeln!(f, "\t\t{}", date).ok();
            }
            writeln!(f, "").ok();
        }

        write!(f, "")
    }
}

impl Classes {
    pub fn new() -> Self {
        let classes_path = Path::new("./data/classes.json");

        if classes_path.exists() {
            let text = std::fs::read_to_string(classes_path).unwrap();
            serde_json::from_str(&text).unwrap()
        } else {
            Self {
                classes: HashMap::new(),
                extras: HashMap::new(),
            }
        }
    }

    pub fn save(&self) -> Result<(), ()> {
        let classes_path = Path::new("./data/classes.json");
        std::fs::write(classes_path, serde_json::to_string_pretty(&self).unwrap()).unwrap();
        Ok(())
    }
}
