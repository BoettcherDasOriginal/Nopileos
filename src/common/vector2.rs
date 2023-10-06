use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

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

impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool { 
        self.x == other.x &&
        self.y == other.y
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x.round(), self.y.round())
    }
}

impl Vector2{
    pub fn new(x: f64,y: f64) -> Self {
        Self { x: x, y: y }
    }

    pub fn as_slice(&mut self) -> [f64; 2] {
        return [self.x,self.y];
    }

    pub fn look_at_rotation(&mut self, target: Vector2) -> i32 {
        let n_current = Vector2::new(0.0,1.0); //Vector2 Up
        let n_target = target.clone() - self.clone();

        let magnitude_c = ((n_current.x * n_current.x) + (n_current.y * n_current.y)).sqrt();
        let magnitude_t = ((n_target.x * n_target.x) + (n_target.y * n_target.y)).sqrt();

        let ctt = n_current * n_target;

        return (((ctt.x + ctt.y) / (magnitude_c * magnitude_t)).acos() * 180.0 / std::f64::consts::PI).round() as i32;
    }

    pub fn in_quad_radius(&mut self, center: Vector2, radius: f64) -> bool {
        let min = center.clone() - Vector2::new(radius, radius);
        let max = center + Vector2::new(radius, radius);

        min.x < self.x && max.x > self.x &&
        min.y < self.y && max.y > self.y
    }
}