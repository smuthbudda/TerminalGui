mod app;
mod database;

use crate::app::AppData;
use ratatui::{style::Stylize, widgets::Widget};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = AppData::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
