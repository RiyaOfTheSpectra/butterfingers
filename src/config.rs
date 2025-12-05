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
    pub real_wpm    : u8,
    pub eff_wpm     : u8,
    pub char_sp_ms  : u16,
    pub ex_wd_sp_ms : u16,
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
            real_wpm    : 30,
            eff_wpm     : 30,
            char_sp_ms  : 20,
            ex_wd_sp_ms : 0,
            tone_hz     : 440,
            level       : 1,
            group_ln    : Len::Constant(5),
            duration    : Duration::from_secs(10),
            usables     : vec!['k', 'm'],
        }
    }

    pub fn to_table(&self, constraints: [Constraint; 2]) -> Table {
        let char_spd_row = Row::new([
            String::from("Dit Speed (in ms)"),
            self.char_sp_ms.to_string()
        ]);

        let tone_row = Row::new([
            String::from("Tone (in Hz)"),
            self.tone_hz.to_string()
        ]);

        let group_ln_row = Row::new([
            String::from("Group Length"),
            self.group_ln.to_string()
        ]);

        let ex_wd_sp_row = Row::new([
            String::from("Extra Word Spacing (in ms)"),
            self.ex_wd_sp_ms.to_string()
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
            [char_spd_row, tone_row, group_ln_row, ex_wd_sp_row, duration_row],
            constraints
        )
    }
}
