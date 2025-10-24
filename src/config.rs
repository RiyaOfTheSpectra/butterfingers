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
    char_spd: u8,
    effc_spd: u8,
    ex_wd_sp: u8,
    tone_hz : u16,
    group_ln: Len,
}

impl Config {
    pub fn init() -> Self {
        Self {
            char_spd: 25,
            effc_spd: 20,
            ex_wd_sp: 0,
            tone_hz : 440,
            group_ln: Len::Constant(5),
        }
    }

    pub fn to_table(&self, constraints: [Constraint; 2]) -> Table {
        let char_spd_row = Row::new([
            String::from("Character Speed (in wpm)"),
            self.char_spd.to_string()
        ]);

        let effc_spd_row = Row::new([
            String::from("Effective Speed (in wpm)"),
            self.effc_spd.to_string()
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
            String::from("Extra Word Spacing"),
            self.ex_wd_sp.to_string()
        ]);

        Table::new(
            [char_spd_row, effc_spd_row, tone_row, group_ln_row, ex_wd_sp_row],
            constraints
        )
    }
}
