use std::string::String;
use std::time::Duration;
use std::fmt;
use ratatui::{
    layout::Constraint,
    widgets::{
        Row,
        Table,
    }
};

#[derive(Debug,Clone,Copy)]
pub enum Len {
    Constant(u8),
    Random(u8, u8),
}

impl fmt::Display for Len {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Len::Constant(x) => write!(f, "{}", x),
            Len::Random(x, y) => write!(f, "{}-{}", x, y)
        }
    }
}

#[derive(Debug,Clone)]
pub struct Config {
    pub trx_wpm     : u16,
    pub chr_wpm     : u16,
    pub level       : u8,
    pub tone_hz     : u16,
    pub group_ln    : Len,
    pub duration    : Duration,
    pub usables     : Vec<char>,
}

impl Config {
    pub fn init() -> Self {
        Self {
            // TODO: Find reasonable values for char_sp_ms & ex_wd_sp_ms.
            trx_wpm     : 20,
            chr_wpm     : 25,
            tone_hz     : 440,
            level       : 1,
            group_ln    : Len::Constant(5),
            duration    : Duration::from_secs(10),
            usables     : vec!['k', 'm'],
        }
    }

    pub fn to_table(&self, constraints: [Constraint; 2]) -> Table {
        let trx_wpm_row = Row::new([
            String::from("Transmission Speed, (in wpm)"),
            self.trx_wpm.to_string()
        ]);

        let tone_row = Row::new([
            String::from("Tone (in Hz)"),
            self.tone_hz.to_string()
        ]);

        let group_ln_row = Row::new([
            String::from("Group Length"),
            self.group_ln.to_string()
        ]);

        let chr_wpm_row = Row::new([
            String::from("Character Speed (in wpm)"),
            self.chr_wpm.to_string()
        ]);

        let duration_row = Row::new([
            String::from("Duration of transmission (in mins)"),
            format!(
                "{}:{}",
                self.duration.as_secs() / 60,
                self.duration.as_secs() % 60
                )
        ]);

        Table::new(
            [trx_wpm_row, chr_wpm_row, tone_row, group_ln_row, duration_row],
            constraints
        )
    }
}
