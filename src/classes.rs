use std::{collections::HashMap, fmt::Display, path::Path};

use serde::{Deserialize, Serialize};

use crate::{absents::Absents, timetable::Subject};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct ClassEntry {
    subject: Subject,
    classes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Classes {
    pub classes: HashMap<Subject, Vec<String>>,
    pub extras: HashMap<Subject, Vec<String>>,
}

impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nSubject\t\tClasses").ok();
        let absents = Absents::new();
        for (sub, entry) in self.classes.iter() {
            write!(f, "{:?}", sub).ok();
            for date in entry {
                write!(f, "\t\t{}", date).ok();
                if let Some(absent_dates) = absents.absents.get(sub) {
                    if absent_dates.contains(date) {
                        write!(f, " (absent 🏃‍♂️)").ok();
                    }
                }
                writeln!(f, "").ok();
            }
            writeln!(f, "").ok();
        }

        writeln!(f, "\nExtra Classes").ok();
        for (sub, entry) in self.extras.iter() {
            write!(f, "{:?}", sub).ok();
            for date in entry {
                write!(f, "\t\t{}", date).ok();
                if let Some(absent_dates) = absents.extras.get(sub) {
                    if absent_dates.contains(date) {
                        write!(f, " (absent 🏃‍♂️)").ok();
                    }
                }
                writeln!(f, "").ok();
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

    pub fn save(&self) -> Result<(), std::io::Error> {
        let classes_path = Path::new("./data/classes.json");
        std::fs::write(classes_path, serde_json::to_string_pretty(&self).unwrap()).unwrap();

        let mut writer = csv::Writer::from_path("./data/class_counts.csv")?;

        for (&sub, entry) in self.classes.iter().chain(self.extras.iter()) {
            writer.serialize(ClassEntry {
                subject: sub,
                classes: entry.len() as u32,
            })?;
        }

        Ok(())
    }

    pub fn last_marked_date(&self) -> Result<String, std::io::Error> {
        let mut last_day = String::new();

        let mut max_dates = 0;

        for (_, dates) in self.classes.iter() {
            if dates.len() > max_dates {
                max_dates = dates.len();
                last_day = dates.last().unwrap().clone();
            }
        }

        Ok(last_day)
    }
}
