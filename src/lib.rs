use clap::Parser;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::{Backend, Constraint, Layout, Terminal},
    Frame,
};
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Config {
    #[arg(default_value_t = String::from("File1"))]
    pub file1: String,
    #[arg(default_value_t = String::from("File2"))]
    pub file2: String,
}

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, _app: &mut App) {
    let _rects = Layout::horizontal([Constraint::Min(5), Constraint::Min(5)]).split(f.size());

    // render_file(f, app, rects[0]);
    // render_scrollbar(f, app, rects[0]);

    // render_file(f, app, rects[1]);
    // render_scrollbar(f, app, rects[1]);
}
