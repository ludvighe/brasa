use std::sync::OnceLock;

use unicode_segmentation::UnicodeSegmentation;

use crate::generics::Vec2;

pub struct Graphic {
    pub sprite: &'static str,
    size: OnceLock<Vec2>,
}

impl Graphic {
    pub const fn new(s: &'static str) -> Graphic {
        Graphic {
            sprite: s,
            size: OnceLock::new(),
        }
    }

    pub fn size(&self) -> Vec2 {
        *self.size.get_or_init(|| {
            let lines: Vec<&str> = self.sprite.lines().collect();
            let height = lines.len() as u16;
            let width = lines
                .iter()
                .map(|line| line.graphemes(true).count())
                .max()
                .unwrap_or(0) as u16;
            Vec2::new(width, height)
        })
    }
}

pub struct Graphics {
    pub log: Graphic,
    pub tree: Graphic,
}

macro_rules! init_graphics {
    () => {
        Graphics {
            log: Graphic::new(include_str!("assets/log.txt")),
            tree: Graphic::new(include_str!("assets/tree.txt")),
        }
    };
}

pub const G: Graphics = init_graphics!();
