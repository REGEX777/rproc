use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame
};

pub struct App{
    counter: u8,
    exit: bool
}

fn main() {
    ratatui::run(|terminal| App::default().run(terminal))    
}


// impl App {
//     pub fn run()
// }