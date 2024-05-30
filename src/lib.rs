use clap::Parser;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
//use log::info;
use ratatui::prelude::{Backend, Buffer, Color, Constraint, Layout, Rect, Style, Terminal, Widget};
use ratatui::widgets::{Block, Borders};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use tui_textarea::TextArea;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    //#[arg(default_value_t = String::from("File1"))]
    pub file1: String,
    //#[arg(default_value_t = String::from("File2"))]
    pub file2: String,
}

#[derive(Debug, Default)]

pub struct App {
    state: AppState,
    file1_widget: FileWidget,
    file2_widget: FileWidget,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum AppState {
    #[default]
    Running, // The app is running
    Quit, // The user has requested the app to quit
}

impl App {
    pub fn new(config: &Config) -> Self {
        Self {
            state: AppState::Running,
            file1_widget: FileWidget::new(&config.file1),
            file2_widget: FileWidget::new(&config.file2),
        }
    }

    /// This is the main event loop for the app.
    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.size()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    const fn is_running(&self) -> bool {
        matches!(self.state, AppState::Running)
    }

    /// Handle any events that have occurred since the last time the app was rendered.
    ///
    /// Currently, this only handles the q key to quit the app.
    fn handle_events(&mut self) -> Result<()> {
        // Ensure that the app only blocks for a period that allows the app to render at
        // approximately 60 FPS (this doesn't account for the time to render the frame, and will
        // also update the app immediately any time an event occurs)
        let timeout = Duration::from_secs_f32(1.0 / 60.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.state = AppState::Quit;
                };
            }
        }
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::Min;
        let [file1_area, file2_area] = Layout::horizontal([Min(0), Min(0)]).areas(area);
        self.file1_widget.render(file1_area, buf);
        self.file2_widget.render(file2_area, buf);
    }
}

/// FileWidget
#[derive(Debug, Default)]
struct FileWidget {
    filename: String,   // name of the log file to view
    lines: Vec<String>, // lines of the log file
}

impl FileWidget {
    pub fn new(filename: &String) -> Self {
        Self {
            filename: filename.clone(),
            lines: vec![],
        }
    }

    fn setup_lines(&mut self) {
        if self.lines.len() > 0 {
            return;
        }
        let file = File::open(self.filename.clone()).expect("no such fool");
        let buf = BufReader::new(file);
        self.lines = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
    }
}

/// Widget impl for `FileWidget`
impl Widget for &mut FileWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.setup_lines();
        //Text::from(self.lines.join("\n")).render(area, buf);
        let mut textarea = TextArea::from(self.lines.clone());
        textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.filename.clone()),
        );
        textarea.widget().render(area, buf);
    }
}
