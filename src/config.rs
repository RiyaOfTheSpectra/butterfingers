use std::string::String;
use std::fmt;
use ratatui::{
    layout::Constraint,
    widgets::{
        Row,
        Table,
    }
};

#[derive(Debug,Clone,Copy)]
enum Len {
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

#[derive(Debug,Clone,Copy)]
pub struct Config {
    pub char_sp_ms  : u16,
    pub ex_wd_sp_ms : u16,
    pub level       : u8,
    pub tone_hz     : u16,
    pub group_ln    : Len,
}

impl Config {
    pub fn init() -> Self {
        Self {
            char_sp_ms  : 25,
            ex_wd_sp_ms : 0,
            tone_hz     : 440,
            level       : 1,
            group_ln    : Len::Constant(5),
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

        Table::new(
            [char_spd_row, tone_row, group_ln_row, ex_wd_sp_row],
            constraints
        )
    }
}
