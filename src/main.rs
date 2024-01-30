#[macro_use]
extern crate crossterm;

mod data_fetch;

mod menu;
use menu::Menu;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let access_key = std::env::var("CANVAS_ACCESS_KEY").unwrap();
    let assignments = data_fetch::get_assignments(access_key).await?;

    let menu = Menu::new(assignments);
    menu.show_menu();

    Ok(())
}
