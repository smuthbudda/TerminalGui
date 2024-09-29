mod app;
mod database;

use ratatui::{style::Stylize, widgets::Widget};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app_data = crate::app::AppData::new();
    let app_result = app_data.run(&mut terminal);
    ratatui::restore();
    app_result
}
