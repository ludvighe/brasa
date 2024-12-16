use crossterm::cursor::MoveTo;
use crossterm::event::read;
use crossterm::event::{self, Event};
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::size;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, ClearType};
use crossterm::ExecutableCommand;
use std::io::{stdout, Stdout, Write};
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;

use crate::generics::Vec2;
use crate::graphics::Graphic;

pub struct Term {
    stdout: Stdout,
}

impl Term {
    pub fn new() -> Term {
        enable_raw_mode().unwrap();
        Term { stdout: stdout() }
    }

    pub fn close(&mut self) {
        self.clear_all();
        disable_raw_mode().unwrap();
    }

    pub fn size() -> Vec2 {
        match size() {
            Ok(value) => Vec2::from(value),
            _ => Vec2::empty(),
        }
    }

    pub fn reset_cursor(&mut self) {
        self.stdout.execute(MoveTo(0, 0)).unwrap();
    }

    pub fn clear_all(&mut self) {
        self.stdout
            .execute(crossterm::terminal::Clear(ClearType::All))
            .unwrap();
        self.stdout.execute(MoveTo(0, 0)).unwrap();
    }

    pub fn write_text(&mut self, at: Vec2, text: impl std::fmt::Display) {
        self.stdout.execute(MoveTo(at.x, at.y)).unwrap();
        write!(self.stdout, "{}", text).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_pixel(
        &mut self,
        at: Vec2,
        bg_color: Option<Color>,
        fg_color: Option<Color>,
        ch: Option<&str>,
    ) {
        self.stdout.execute(MoveTo(at.x, at.y)).unwrap();
        if let Some(bg) = bg_color {
            self.stdout.execute(SetBackgroundColor(bg)).unwrap();
        }
        if let Some(fg) = fg_color {
            self.stdout.execute(SetForegroundColor(fg)).unwrap();
        }
        write!(self.stdout, "{}", ch.unwrap_or(" ")).unwrap();
        self.stdout.execute(ResetColor).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_pixel_bg(&mut self, at: Vec2, color: Color) {
        self.stdout.execute(MoveTo(at.x, at.y)).unwrap();
        self.stdout.execute(SetBackgroundColor(color)).unwrap();
        write!(self.stdout, " ").unwrap();
        self.stdout.execute(ResetColor).unwrap();
    }

    pub fn draw(&mut self, at: Vec2, graphic: &str) {
        for (y, line) in graphic.lines().enumerate() {
            for (x, c) in line.graphemes(true).enumerate() {
                if c == " " {
                    continue;
                }

                self.set_pixel(
                    Vec2::new(at.x + x as u16, at.y + y as u16),
                    None,
                    None,
                    Some(c),
                );
            }
        }
    }

    pub fn draw_graphic(&mut self, at: Vec2, graphic: Graphic) {
        self.draw(at, graphic.sprite);
    }

    pub fn read_event(&self, timeout_ms: u64) -> Option<Event> {
        if event::poll(Duration::from_millis(timeout_ms)).unwrap() {
            Some(read().unwrap())
        } else {
            None
        }
    }
}
