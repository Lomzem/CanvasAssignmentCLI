#[macro_use]
extern crate crossterm;

use chrono::{Datelike, Duration, Local, NaiveDate};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{io, process::Command};

use data_fetch::{Assignment, AssignmentManipulation};
mod data_fetch;

fn show_menu(assignments: &Vec<Assignment>, due_date: NaiveDate) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut selection: usize = 0;
    let mut date = due_date;
    enable_raw_mode().unwrap();

    loop {
        let filtered: &Vec<&Assignment> = &assignments
            .iter()
            .filter(|a| a.info.as_ref().unwrap().due_at.eq(&date))
            .collect();

        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Print(format!(
                "Due on {} ({} days)",
                &date.format("%B %e, %Y"),
                date.clone().num_days_from_ce() - Local::now().date_naive().num_days_from_ce()
            ))
        )
        .unwrap();

        for (index, elem) in filtered.iter().enumerate() {
            if selection == index {
                execute!(
                    stdout,
                    cursor::MoveTo(0, (index + 1) as u16),
                    Print(format!("{} <--", &elem.title))
                )
                .unwrap();
            } else {
                execute!(
                    stdout,
                    cursor::MoveTo(0, (index + 1) as u16),
                    Print(format!("{}", &elem.title))
                )
                .unwrap();
            }
        }

        let event = read()?;
        if event == Event::Key(KeyCode::Char('q').into()) {
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            disable_raw_mode().unwrap();
            break;
        } else if event == Event::Key(KeyCode::Char('j').into()) {
            selection = selection.saturating_add(1).min(filtered.len() - 1);
        } else if event == Event::Key(KeyCode::Char('k').into()) {
            selection = selection.saturating_sub(1);
        } else if event == Event::Key(KeyCode::Char('h').into()) {
            date = date - Duration::days(1);
        } else if event == Event::Key(KeyCode::Char('l').into()) {
            date = date + Duration::days(1);
        } else if event == Event::Key(KeyCode::Char('o').into()) {
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            // execute!(
            //     stdout,
            //     Clear(ClearType::All),
            //     cursor::MoveTo(0, 0),
            //     Print(&filtered.get(selection).unwrap().url)
            // )
            // .unwrap();
            Command::new("brave.exe")
                .arg(&filtered.get(selection).unwrap().url)
                .spawn()
                .unwrap();
            disable_raw_mode().unwrap();
            break;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let access_key = std::env::var("CANVAS_ACCESS_KEY").unwrap();
    let assignments = data_fetch::get_assignments(access_key).await?;

    show_menu(&assignments, assignments.first_date()).unwrap();

    Ok(())
}
