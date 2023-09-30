use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Debug)]
pub struct Vector2{
    pub x: f64,
    pub y: f64,
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {x: self.x * other.x, y: self.y * other.y}
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {x: self.x / other.x, y: self.y / other.y}
    }
}

impl Vector2{
    pub fn new(x: f64,y: f64) -> Self {
        Self { x: x, y: y }
    }

    pub fn as_slice(&mut self) -> [f64; 2] {
        return [self.x,self.y];
    }
}