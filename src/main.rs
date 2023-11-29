#[macro_use]
extern crate crossterm;

use chrono::{Datelike, Local, NaiveDate};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io;

use data_fetch::{Assignment, AssignmentManipulation};
mod data_fetch;

fn show_menu(assignments: &Vec<Assignment>, due_date: NaiveDate) -> io::Result<()> {
    println!(
        "Due on {} ({} days)",
        &due_date.format("%B %e, %Y"),
        due_date.clone().num_days_from_ce() - Local::now().date_naive().num_days_from_ce()
    );

    let mut stdout = io::stdout();
    let mut selection: usize = 0;
    enable_raw_mode().unwrap();

    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

        for (index, elem) in assignments.iter().enumerate() {
            if selection == index {
                // print!("{}) {} <--", index, &elem.title);
                execute!(
                    stdout,
                    cursor::MoveTo(0, index.try_into().unwrap()),
                    Print(format!("{}) {} <--", index, &elem.title))
                )
                .unwrap();
            } else {
                execute!(
                    stdout,
                    cursor::MoveTo(0, index.try_into().unwrap()),
                    Print(format!("{}) {}", index, &elem.title))
                )
                .unwrap();
            }
        }

        let event = read()?;
        if event == Event::Key(KeyCode::Char('q').into()) {
            disable_raw_mode().unwrap();
            break;
        } else if event == Event::Key(KeyCode::Char('j').into()) {
            selection = selection.saturating_add(1).min(assignments.len() - 1);
        } else if event == Event::Key(KeyCode::Char('k').into()) {
            selection = selection.saturating_sub(1);
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
