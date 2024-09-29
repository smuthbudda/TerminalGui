use crate::database::DataConnection;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Line, Stylize, Widget};
use ratatui::symbols::border;
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::{fs, io};

#[derive(Debug)]
pub struct AppData {
    mode: Mode,
    files: Vec<FileData>,
    first_load: bool,
    data_connection: DataConnection,
}

impl AppData {
    pub fn new() -> AppData {
        AppData {
            mode: Mode::Running,
            first_load: false,
            files: vec![],
            data_connection: DataConnection::new(),
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.is_running() {
            terminal.draw(|frame| {
                self.draw(frame);
            })?;

            // Set the app mode to quit in case q key press
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.exit();
                }
            }

            // Initialise the original data
            if self.first_load {
                self.initial_load();
            }
        }
        Ok(())
    }

    /// Check if the app is running
    fn is_running(&self) -> bool {
        !matches!(self.mode, Mode::Quit | Mode::Destroy)
    }

    /// Draws the UI to the screen
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// Loads the initial data into the app state
    fn initial_load(&mut self) {
        self.files = get_files("/");
        self.first_load = true;
    }

    // fn handle_events(&mut self) -> io::Result<()> {
    //     match event::read()? {
    //         // it's important to check that the event is a key press event as
    //         // crossterm also emits key release and repeat events on Windows.
    //         Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
    //             self.handle_key_event(key_event)
    //         }
    //         _ => {}
    //     };
    //     Ok(())
    // }

    fn exit(&mut self) {
        self.mode = Mode::Quit;
    }
}

impl Widget for &AppData {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Terminal Test App ".bold());
        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK)
            .yellow()
            .on_gray();

        Paragraph::new("Files:")
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[derive(Debug)]
enum Mode {
    Running,
    Destroy,
    Quit,
}

#[derive(Debug)]
struct FileData {
    name: String,
    file_path: String,
}

impl FileData {
    /// Creates a new file data instance
    pub fn new(name: String, file_path: String) -> FileData {
        FileData { name, file_path }
    }
}

/// Gets the list of file in the current directory
fn get_files(file_path: &str) -> Vec<FileData> {
    let mut result: Vec<FileData> = vec![];

    let paths = fs::read_dir(file_path).unwrap();

    for path in paths {
        if path.is_ok() {
            let name = path.unwrap().file_name().to_str().unwrap().to_owned();
            let file = FileData::new(name, file_path.to_string());
            result.push(file);
        }
    }
    result
}
