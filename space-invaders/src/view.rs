use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, next_frame: &Frame, force_refresh: bool) {
    if force_refresh {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, column) in next_frame.iter().enumerate() {
        for (y, cell) in column.iter().enumerate() {
            if *cell != last_frame[x][y] || force_refresh {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *cell);
            }
        }
    }

    stdout.flush().unwrap();
}
