use std::vec::Vec;
use std::string::String;
use std::time::Duration;
use std::io;
use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
    poll,
};
use ratatui::{
    widgets::{
        Block,
        Paragraph,
        Widget,
        Row,
        Table,
    },
    layout::{
        Layout,
        Rect,
        Direction,
        Constraint,
    },
    buffer::Buffer,
    style::Stylize,
    symbols::border,
    text::{Line, Text,},
    DefaultTerminal,
    Frame,
};

use crate::config::*;

#[derive(Debug,Clone,Copy)]
enum Mode {
    Checking,
    Testing,
    Waiting,
    Controls,
}

#[derive(Debug,Clone)]
pub struct tutor {
    letters: String,
    mode: Mode,
    exit: bool,
}

impl tutor {
    pub fn init() -> Self {
        tutor {
            letters : String::from(""),
            mode: Mode::Waiting,
            exit : false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if poll(Duration::from_millis(100))? {
                let _ = self.handle_events();
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()?  {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode{
            Mode::Waiting => match key_event.code {
                KeyCode::Char('m') => self.to_mode(Mode::Controls),
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char(' ') => self.to_mode(Mode::Testing),
                _ => {}
            }
            Mode::Testing => {
                match key_event.code {
                    KeyCode::Char(x) => self.letters.push(x),
                    KeyCode::Esc => self.to_mode(Mode::Waiting),
                    KeyCode::Backspace => _ = self.letters.pop(),
                    _ => {}
                }
            }
            Mode::Controls => {
                match key_event.code {
                    KeyCode::Esc => self.to_mode(Mode::Waiting),
                    _ => {}
                }
            },
            Mode::Checking => {},
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn to_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

impl Widget for &tutor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inside = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(6),
                Constraint::Percentage(47),
                Constraint::Percentage(47),
            ])
            .split(area);

        let bottom = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ])
            .split(inside[2]);

        let title = Line::from(" Uncle Sam’s Training Terminal ".bold());

        let instructions = Line::from(vec![
            " Controls ".into(),
            "<M> ".blue().bold(),
            " Start ".into(),
            "<SPACE>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let top_line = Block::bordered()
            .title(title.left_aligned())
            .title_bottom(instructions.right_aligned())
            .border_set(border::THICK)
            .render(inside[0], buf);

        let input_block = Block::bordered()
            .title(Line::from("Demoed").left_aligned())
            .border_set(border::PLAIN);

        let result_block = Block::bordered()
            .title(Line::from("Result").left_aligned())
            .border_set(border::PLAIN);

        let comment_block = Block::bordered()
            .title(Line::from("Comments").left_aligned())
            .border_set(border::PLAIN);

        let control_block = Block::bordered()
            .title(Line::from("Controls").left_aligned())
            .border_set(border::PLAIN);

        let input_text = Text::from(
            vec![
                Line::from(self.letters.clone().yellow()),
        ]);

        let result_text = Text::from(vec![]);

        let comment_text = Text::from(vec![]);

        let control_table = Config::init();

        Paragraph::new(input_text)
            .left_aligned()
            .block(input_block)
            .render(inside[1], buf);

        Paragraph::new(result_text)
            .left_aligned()
            .block(result_block)
            .render(bottom[0], buf);

        Paragraph::new(comment_text)
            .left_aligned()
            .block(comment_block)
            .render(bottom[1], buf);

        control_table.to_table([Constraint::Percentage(90), Constraint::Percentage(10)])
            .block(control_block)
            .render(bottom[2], buf);
    }
}
