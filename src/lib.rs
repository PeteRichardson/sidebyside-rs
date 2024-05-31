use clap::Parser;
use color_eyre::Result;
use crossterm::event;
use ratatui::prelude::{Backend, Buffer, Constraint, Layout, Rect, Terminal, Widget};
use std::time::Duration;
use tui_textarea::{Input, Key};
mod filewidget;
use filewidget::FileWidget;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    pub file1: String,
    pub file2: String,
}

#[derive(Debug, Default)]

pub struct App<'a> {
    state: AppState,
    widgets: Vec<FileWidget<'a>>,
    active_widget: usize,
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
            widgets: vec![
                FileWidget::new(&config.file1),
                FileWidget::new(&config.file2),
            ],
            active_widget: 0,
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
        if event::poll(timeout)? {
            if event::poll(std::time::Duration::from_millis(16))? {
                let input = event::read()?.into();
                match input {
                    Input {
                        key: Key::Char('q'),
                        ..
                    }
                    | Input { key: Key::Esc, .. } => self.state = AppState::Quit,
                    Input { key: Key::Tab, .. } => {
                        self.active_widget = (self.active_widget + 1) % self.widgets.len();
                    }

                    _ => self.widgets[self.active_widget].handle_events(input)?,
                }
            }
        }
        Ok(())
    }
}

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::Min;
        // calculate rects where widgets should be rendered
        assert!(self.widgets.len() == 2);
        let widget_areas: Vec<Rect> = Layout::horizontal([Min(0), Min(0)])
            .areas::<2>(area)
            .to_vec();
        for (i, w) in self.widgets.iter_mut().enumerate() {
            w.active = i == self.active_widget;
            w.render(widget_areas[i], buf);
        }
    }
}
