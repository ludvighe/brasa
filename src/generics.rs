use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    pub fn from(tuple: (u16, u16)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
    pub fn empty() -> Self {
        Self::new(0, 0)
    }
    pub fn x(&self) -> Self {
        Vec2::new(self.x, 0)
    }
    //pub fn y(&self) -> Self {
    //    Vec2::new(0, self.y)
    //}
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<u16> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: u16) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<u16> for Vec2 {
    type Output = Self;

    fn div(self, scalar: u16) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}
