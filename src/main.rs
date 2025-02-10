use std::{fs, path::Path};

pub mod attendance;
pub mod classes;
pub mod timetable;

use attendance::Attendance;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use classes::Classes;
use inquire::{Confirm, DateSelect, MultiSelect, Select};
use timetable::TimeTable;

fn main() {
    let time_table = match load_time_table() {
        Some(value) => value,
        None => return,
    };

    let mut attendance = match load_attendance() {
        Some(value) => value,
        None => return,
    };

    let mut classes = load_classes();

    println!("Classes Counter in Rust");
    println!("This CLI allows you to keep track of total no. of classes each subject had in your college!\n");

    let local: DateTime<Local> = Local::now();
    let today = local.format("%d %b %Y");
    let day = local.weekday();

    let todays_subjects = match time_table.subjects_on(local.weekday()) {
        Some(subs) => subs,
        None => {
            eprintln!("No classes today, go sleep!");
            return;
        }
    };

    println!("{}", today);
    println!("Today is {}", day);
    println!("Today's subjects are {:?}", todays_subjects);

    println!();

    let options: Vec<String> = vec![
        format!(
            "1. Mark today's classes ({} {})",
            local.weekday(),
            today.to_string()
        ),
        "2. Mark day's classes (Select Date)".to_string(),
        "3. Check Class Counts".to_string(),
        "4. Check Classes".to_string(),
        "5. Save".to_string(),
        "6. Exit".to_string(),
    ];

    loop {
        let option = match Select::new("What do you want to do?", options.clone()).prompt() {
            Ok(choice) => choice,
            Err(_) => {
                eprintln!("Wait how did you...?");
                return;
            }
        };

        match option.as_str() {
            x if x == options[0] => {
                mark_classes(
                    todays_subjects,
                    &mut attendance,
                    &mut classes,
                    Local::now().date_naive(),
                );
            }

            x if x == options[1] => {
                let selected_date = DateSelect::new("Which date to mark classes of?")
                    .with_week_start(chrono::Weekday::Mon)
                    .prompt();

                let subjects = match selected_date {
                    Ok(d) => match time_table.subjects_on(d.weekday()) {
                        Some(subs) => subs,
                        None => {
                            eprintln!("No classes, go sleep!");
                            return;
                        }
                    },
                    Err(_) => {
                        eprintln!("No date selected?!");
                        return;
                    }
                };

                let edited_subjects =
                    match MultiSelect::new("Which subjects to mark?", subjects.clone())
                        .with_all_selected_by_default()
                        .prompt()
                    {
                        Ok(subs) => subs,
                        Err(_) => {
                            eprintln!("What happened? Cancelled or something?");
                            return;
                        }
                    };

                mark_classes(
                    &edited_subjects,
                    &mut attendance,
                    &mut classes,
                    selected_date.expect("Didn't selected any date, also didn't returned"),
                );
            }

            x if x == options[2] => {
                println!("{}", attendance);
                println!("Extra Classes");
                for (sub, dates) in classes.extras.iter() {
                    println!("{}\t\t{}", sub, dates.len());
                }
            }

            x if x == options[3] => {
                println!("{}", classes);
            }

            x if x == options[4] => {
                if let Ok(save) = Confirm::new("Sure to Save (y/n)?").prompt() {
                    if save {
                        attendance.save().ok();
                        classes.save().ok();
                    }
                }
            }

            x if x == options[5] => {
                println!("Thanks for using, exiting...");
                break;
            }

            _ => {
                eprintln!("Wait why did you...?");
                return;
            }
        }
    }
}

fn mark_classes(
    subjects: &Vec<timetable::Subject>,
    attendance: &mut Attendance,
    classes: &mut Classes,
    date: NaiveDate,
) {
    for &sub in subjects {
        if let Some((_, entry)) = attendance
            .subjects
            .iter_mut()
            .find(|&&mut (subject, _)| subject == sub)
        {
            entry.classes += 1;
            classes
                .classes
                .entry(sub)
                .and_modify(|x| x.push(date.format("%d %b %Y, %a").to_string()))
                .or_insert(Vec::from([date.format("%d %b %Y, %a").to_string()]));
        }
    }

    println!(
        "Marked +1 in {:?} on {}",
        subjects,
        date.format("%d %b %Y, %a")
    );
}

fn load_attendance() -> Option<Attendance> {
    let attendance = match Attendance::new() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading attendance data -> {}", e);
            return None;
        }
    };
    Some(attendance)
}

fn load_time_table() -> Option<TimeTable> {
    let time_table_path = Path::new("./data/timetable.json");
    let time_table_data = match fs::read_to_string(time_table_path) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Error opening time table");
            return None;
        }
    };
    let time_table = match TimeTable::new(&time_table_data) {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Error parsing time table -> {}", e);
            return None;
        }
    };
    Some(time_table)
}

fn load_classes() -> Classes {
    Classes::new()
}
