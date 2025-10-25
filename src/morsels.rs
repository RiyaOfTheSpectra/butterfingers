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
use crate::keying::*;

#[derive(Debug,Clone,Copy)]
enum Mode {
    Checking,
    Testing,
    Waiting,
    Control,
}

#[derive(Debug,Clone)]
pub struct tutor {
    input_letters: String,
    mode: Mode,
    config: Config,
    sequence: Vec<char>,
    exit: bool,
}

impl tutor {
    pub fn init() -> Self {
        tutor {
            input_letters : String::from(""),
            mode: Mode::Waiting,
            config: Config::init(),
            sequence: vec!['k', 'm'],
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
            Mode::Waiting => self.handle_waiting_key(key_event),
            Mode::Testing => self.handle_testing_key(key_event),
            Mode::Control => self.handle_control_key(key_event),
            Mode::Checking => {},
        }
    }

    fn handle_waiting_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('m') => self.to_mode(Mode::Control),
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(' ') => self.to_mode(Mode::Testing),
            _ => {}
        }
    }

    fn handle_testing_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(x) => self.input_letters.push(x),
            KeyCode::Esc => self.to_mode(Mode::Waiting),
            KeyCode::Backspace => _ = self.input_letters.pop(),
            _ => {}
        }
    }

    fn handle_control_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.to_mode(Mode::Waiting),
            _ => {}
        }
    }

    fn to_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn exit(&mut self) {
        self.exit = true;
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
            " Control ".into(),
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
                Line::from(self.input_letters.clone().yellow()),
        ]);

        let result_text = Text::from(vec![]);

        let comment_text = Text::from(vec![]);

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

        self.config.to_table([Constraint::Percentage(90), Constraint::Percentage(10)])
            .block(control_block)
            .render(bottom[2], buf);
    }
}
