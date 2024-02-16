use chrono::{Datelike, Duration, Local, NaiveDate};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::disable_raw_mode;
use crossterm::{
    cursor,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::data_fetch::{Assignment, FirstDate};
use std::io;

pub struct Menu {
    assignments: Vec<Assignment>,
}

impl Menu {
    pub fn new(assignments: Vec<Assignment>) -> Self {
        Self { assignments }
    }

    pub fn quit(&self) -> Result<(), std::io::Error> {
        let result = execute!(
            io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            Clear(ClearType::All),
            cursor::Show,
            cursor::MoveTo(0, 0),
        );
        disable_raw_mode()?;
        result?;
        Ok(())
    }

    fn assignments_on_date(&self, due_date: &NaiveDate) -> Vec<&Assignment> {
        // Returns a vec of assignments that only are due on
        // due_date from the Menu struct's "assignments" field
        self.assignments
            .iter()
            .filter(|a| a.info.as_ref().unwrap().due_at.eq(due_date))
            .collect()
    }

    pub fn show_menu(&self) {
        let mut stdout = io::stdout();
        let mut date = self.assignments.first_date().to_owned();
        crossterm::terminal::enable_raw_mode().unwrap();

        let mut selection: usize = 0;

        loop {
            execute!(
                stdout,
                Clear(ClearType::All),
                cursor::Hide,
                cursor::MoveTo(0, 0),
                Print(format!(
                    "Due on {} ({} days)",
                    &date.format("%B %e, %Y"),
                    date.num_days_from_ce() - Local::now().date_naive().num_days_from_ce()
                ))
            )
            .unwrap();

            let filtered_assignments = self.assignments_on_date(&date);

            for (index, elem) in filtered_assignments.iter().enumerate() {
                let course = &elem
                    .course
                    .split_whitespace()
                    .take(2)
                    .collect::<Vec<&str>>()
                    .join(" ");

                if selection == index {
                    execute!(
                        stdout,
                        cursor::MoveTo(0, (index + 1) as u16),
                        Print(format!("{} - {} <--", &course, &elem.title))
                    )
                    .unwrap();
                } else {
                    execute!(
                        stdout,
                        cursor::MoveTo(0, (index + 1) as u16),
                        Print(format!("{} - {}", &course, &elem.title))
                    )
                    .unwrap();
                }
            }
            let event = crossterm::event::read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    self.quit().unwrap();
                    break;
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    ..
                }) => {
                    selection = selection
                        .saturating_add(1)
                        .min(filtered_assignments.len() - 1)
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    ..
                }) => selection = selection.saturating_sub(1),

                Event::Key(KeyEvent {
                    code: KeyCode::Char('h'),
                    ..
                }) => date -= Duration::days(1),

                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    ..
                }) => date += Duration::days(1),

                Event::Key(KeyEvent {
                    code: KeyCode::Char('o'),
                    ..
                }) => {
                    self.quit().unwrap();
                    std::process::Command::new("brave.exe")
                        .arg(&filtered_assignments.get(selection).unwrap().url)
                        .spawn()
                        .unwrap();
                    break;
                }
                _ => {}
            }
        }
    }
}
