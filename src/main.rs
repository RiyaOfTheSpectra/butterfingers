mod morsels;
mod config;
mod keying;

use std::io;
use morsels::Tutor;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = Tutor::init().run(&mut terminal);
    ratatui::restore();
    app_result
}
