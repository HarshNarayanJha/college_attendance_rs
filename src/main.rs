use std::{fs, path::Path};

pub mod absents;
pub mod classes;
pub mod timetable;

use absents::Absents;
use chrono::{DateTime, Datelike, Local, NaiveDate};
use classes::Classes;
use inquire::{Confirm, DateSelect, MultiSelect, Select};
use timetable::{Subject, TimeTable};

fn main() {
    let time_table = match load_time_table() {
        Some(value) => value,
        None => return,
    };

    let mut classes = load_classes();
    let mut absents = load_absents();

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
        "3. Mark Absents (Select Date)".to_string(),
        "4. Check Class Counts".to_string(),
        "5. Check Classes".to_string(),
        "6. Check Absents".to_string(),
        "7. Save".to_string(),
        "8. Exit".to_string(),
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
                    &mut classes,
                    selected_date.expect("Didn't selected any date, also didn't returned"),
                );
            }

            x if x == options[2] => {
                let selected_date = DateSelect::new("Which date to mark absents of?")
                    .with_week_start(chrono::Weekday::Mon)
                    .with_max_date(Local::now().date_naive())
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
                    match MultiSelect::new("Which subjects to mark?", subjects.clone()).prompt() {
                        Ok(subs) => subs,
                        Err(_) => {
                            eprintln!("What happened? Cancelled or something?");
                            return;
                        }
                    };

                mark_absents(&edited_subjects, &mut absents, selected_date.unwrap());
            }

            x if x == options[3] => {
                for (sub, entry) in classes.classes.iter() {
                    println!("{:?}\t\t{}", sub, entry.len());
                }
                println!("\nExtra Classes");
                for (sub, entry) in classes.extras.iter() {
                    println!("{:?}\t\t{}", sub, entry.len());
                }
            }

            x if x == options[4] => {
                println!("{}", classes);
            }

            x if x == options[5] => {
                println!("{}", absents);
            }

            x if x == options[6] => {
                if let Ok(save) = Confirm::new("Sure to Save (y/n)?").prompt() {
                    if save {
                        classes.save().ok();
                        absents.save().ok();
                    }
                }
            }

            x if x == options[7] => {
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
    classes: &mut Classes,
    date: NaiveDate,
) {
    for &sub in subjects {
        classes
            .classes
            .entry(sub)
            .and_modify(|x| x.push(date.format("%d %b %Y, %a").to_string()))
            .or_insert(Vec::from([date.format("%d %b %Y, %a").to_string()]));
    }

    println!(
        "Marked +1 in {:?} on {}",
        subjects,
        date.format("%d %b %Y, %a")
    );
}

fn mark_absents(subjects: &Vec<Subject>, absents: &mut Absents, on: NaiveDate) {
    for &sub in subjects {
        absents
            .absents
            .entry(sub)
            .and_modify(|x| x.push(on.format("%d %b %Y, %a").to_string()))
            .or_insert(Vec::from([on.format("%d %b %Y, %a").to_string()]));
    }
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

fn load_absents() -> Absents {
    Absents::new()
}
