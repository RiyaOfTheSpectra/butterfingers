mod morsels;
mod config;
 
use std::io;
use morsels::tutor;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = tutor::init().run(&mut terminal);
    ratatui::restore();
    app_result
}
