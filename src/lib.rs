use clap::Parser;

use color_eyre::Result;
use crossterm::event;
use ratatui::prelude::{Backend, Buffer, Color, Constraint, Layout, Rect, Style, Terminal, Widget};
use ratatui::widgets::{Block, Borders};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    pub file1: String,
    pub file2: String,
}

#[derive(Debug, Default)]

pub struct App<'a> {
    state: AppState,
    file1_widget: FileWidget<'a>,
    file2_widget: FileWidget<'a>,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum AppState {
    #[default]
    Running, // The app is running
    Quit, // The user has requested the app to quit
}

impl App<'_> {
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
    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 60.0);
        let textarea = &mut self.file1_widget.textarea;
        if event::poll(timeout)? {
            if event::poll(std::time::Duration::from_millis(16))? {
                let input = event::read()?.into();
                match input {
                    Input {
                        key: Key::Char('q'),
                        ..
                    }
                    | Input { key: Key::Esc, .. } => self.state = AppState::Quit,
                    Input {
                        key: Key::Char('h'),
                        ..
                    }
                    | Input { key: Key::Left, .. } => textarea.move_cursor(CursorMove::Back),
                    Input {
                        key: Key::Char('j'),
                        ..
                    }
                    | Input { key: Key::Down, .. } => textarea.move_cursor(CursorMove::Down),
                    Input {
                        key: Key::Char('k'),
                        ..
                    }
                    | Input { key: Key::Up, .. } => textarea.move_cursor(CursorMove::Up),
                    Input {
                        key: Key::Char('l'),
                        ..
                    }
                    | Input {
                        key: Key::Right, ..
                    } => textarea.move_cursor(CursorMove::Forward),
                    Input {
                        key: Key::Char('w'),
                        ..
                    } => textarea.move_cursor(CursorMove::WordForward),
                    Input {
                        key: Key::Char('b'),
                        ctrl: false,
                        ..
                    } => textarea.move_cursor(CursorMove::WordBack),
                    Input {
                        key: Key::Char('^'),
                        ..
                    } => textarea.move_cursor(CursorMove::Head),
                    Input {
                        key: Key::Char('$'),
                        ..
                    } => textarea.move_cursor(CursorMove::End),
                    Input {
                        key: Key::Char('g'),
                        ctrl: false,
                        ..
                    }
                    | Input { key: Key::Home, .. } => textarea.move_cursor(CursorMove::Top),
                    Input {
                        key: Key::Char('G'),
                        ctrl: false,
                        ..
                    }
                    | Input { key: Key::End, .. } => textarea.move_cursor(CursorMove::Bottom),
                    Input {
                        key: Key::Char('e'),
                        ctrl: true,
                        ..
                    } => textarea.scroll((1, 0)),
                    Input {
                        key: Key::Char('y'),
                        ctrl: true,
                        ..
                    } => textarea.scroll((-1, 0)),
                    Input {
                        key: Key::Char('d'),
                        ctrl: true,
                        ..
                    } => textarea.scroll(Scrolling::HalfPageDown),
                    Input {
                        key: Key::Char('u'),
                        ctrl: true,
                        ..
                    } => textarea.scroll(Scrolling::HalfPageUp),
                    Input {
                        key: Key::Char('f'),
                        ctrl: true,
                        ..
                    }
                    | Input {
                        key: Key::PageDown, ..
                    } => textarea.scroll(Scrolling::PageDown),
                    Input {
                        key: Key::Char('b'),
                        ctrl: true,
                        ..
                    }
                    | Input {
                        key: Key::PageUp, ..
                    } => textarea.scroll(Scrolling::PageUp),
                    _ => (),
                }
            }
            // if let Event::Key(key) = event::read()? {
            //     if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
            //         self.state = AppState::Quit;
            //     };
            // }
        }
        Ok(())
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::Min;
        let [file1_area, file2_area] = Layout::horizontal([Min(0), Min(0)]).areas(area);
        self.file1_widget.render(file1_area, buf);
        self.file2_widget.render(file2_area, buf);
    }
}

/// FileWidget
#[derive(Debug, Default)]
struct FileWidget<'a> {
    filename: String, // name of the log file to view
    textarea: TextArea<'a>,
}

impl FileWidget<'_> {
    pub fn new(filename: &String) -> Self {
        let file = File::open(filename.clone()).expect("no such file");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        Self {
            filename: filename.clone(),
            textarea: TextArea::new(lines.clone()),
        }
    }
}

/// Widget impl for `FileWidget`
impl Widget for &mut FileWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.textarea
            .set_line_number_style(Style::default().fg(Color::DarkGray));
        self.textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.filename.clone()),
        );
        self.textarea.widget().render(area, buf);
    }
}
