use crossterm::cursor::MoveTo;
use crossterm::event::read;
use crossterm::event::{self, Event};
use crossterm::style::{
    Attribute, Color, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
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
        let mut stdout = stdout();
        stdout.execute(crossterm::cursor::Hide).unwrap();
        Term { stdout }
    }

    pub fn close(&mut self) {
        self.clear_all();
        self.stdout.execute(crossterm::cursor::Show).unwrap();
        disable_raw_mode().unwrap();
    }

    pub fn size() -> Vec2 {
        match size() {
            Ok(value) => Vec2::from(value),
            _ => Vec2::empty(),
        }
    }

    //pub fn reset_cursor(&mut self) {
    //    self.stdout.execute(MoveTo(0, 0)).unwrap();
    //}

    pub fn clear_all(&mut self) {
        self.stdout
            .execute(crossterm::terminal::Clear(ClearType::All))
            .unwrap();
        self.stdout.execute(MoveTo(0, 0)).unwrap();
    }

    pub fn set_attr_bold(&mut self) {
        self.stdout.execute(SetAttribute(Attribute::Bold)).unwrap();
    }
    pub fn set_attr_reset(&mut self) {
        self.stdout.execute(SetAttribute(Attribute::Reset)).unwrap();
    }

    pub fn write_text(&mut self, at: Vec2, text: impl std::fmt::Display) {
        self.stdout.execute(MoveTo(at.x, at.y)).unwrap();
        write!(self.stdout, "{}", text).unwrap();
        self.stdout.flush().unwrap();
    }
    pub fn write_bold_text(&mut self, at: Vec2, text: impl std::fmt::Display) {
        self.stdout.execute(MoveTo(at.x, at.y)).unwrap();
        self.set_attr_bold();
        write!(self.stdout, "{}", text).unwrap();
        self.set_attr_reset();
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

    pub fn draw_text_bubble(&mut self, at: Vec2, text: impl std::fmt::Display) {
        let string = text.to_string();
        let lines: Vec<&str> = string.lines().collect();
        let max_len = string.lines().map(|l| l.len()).max().unwrap_or(0);
        let padding: u16 = 0;
        let outline_color = Some(Color::AnsiValue(22));

        let size = Vec2::new(
            max_len as u16 + (padding * 2) + 2,
            lines.len() as u16 + (padding * 2) + 1,
        );

        self.set_pixel(at, None, outline_color, Some("┏"));
        self.set_pixel(at + Vec2::new(size.x, 0), None, outline_color, Some("┓"));
        self.set_pixel(at + Vec2::new(0, size.y), None, outline_color, Some("┗"));
        self.set_pixel(at + size, None, outline_color, Some("┛"));

        for x in 1..size.x {
            self.set_pixel(at + Vec2::new(x, 0), None, outline_color, Some("━"));
            self.set_pixel(at + Vec2::new(x, size.y), None, outline_color, Some("━"));
        }

        for y in 1..size.y {
            self.set_pixel(at + Vec2::new(0, y), None, outline_color, Some("┃"));
            self.set_pixel(at + Vec2::new(size.x, y), None, outline_color, Some("┃"));
        }

        for (i, line) in lines.iter().enumerate() {
            self.write_bold_text(at + Vec2::new(1, i as u16 + 1), *line);
        }
    }

    pub fn set_pixel_bg(&mut self, at: Vec2, color: Color) {
        self.stdout.execute(MoveTo(at.x, at.y)).unwrap();
        self.stdout.execute(SetBackgroundColor(color)).unwrap();
        write!(self.stdout, " ").unwrap();
        self.stdout.execute(ResetColor).unwrap();
    }

    pub fn draw(&mut self, at: Vec2, graphic: &str, color: Color) {
        for (y, line) in graphic.lines().enumerate() {
            for (x, c) in line.graphemes(true).enumerate() {
                if c == " " {
                    continue;
                }

                self.set_pixel(
                    Vec2::new(at.x + x as u16, at.y + y as u16),
                    None,
                    Some(color),
                    Some(c),
                );
            }
        }
    }

    pub fn draw_graphic(&mut self, at: Vec2, graphic: Graphic, color: Color) {
        self.draw(at, graphic.sprite, color);
    }

    pub fn read_event(&self, timeout_ms: u64) -> Option<Event> {
        if event::poll(Duration::from_millis(timeout_ms)).unwrap() {
            Some(read().unwrap())
        } else {
            None
        }
    }
}
