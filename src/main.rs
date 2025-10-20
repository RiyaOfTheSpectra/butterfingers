mod morsels;

use std::io;
use morsels::tutor;
use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text,},
    widgets::{Block, Paragraph, Widget,},
    DefaultTerminal,
    Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = tutor::init().run(&mut terminal);
    ratatui::restore();
    app_result
}
