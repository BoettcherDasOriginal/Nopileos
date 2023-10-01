use super::vector2::Vector2;

use egui::remap;
use std::f64::consts::TAU;

pub struct Triangle {
    pub pos: Vector2,
    pub radius: f64,
    rotation: i32,
}

impl Triangle {
    pub fn new(pos: Vector2,radius: f64,rotation: i32) -> Self {
        Self { pos: pos, radius: radius, rotation: rotation }
    }

    pub fn get_points(&mut self) ->  Vec<[f64; 2]> {
        let n = 360;
        let mut triangle_points: Vec<[f64; 2]> = vec![];
        for i in 0..n {
            let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
            if i == get_rotation(210, self.rotation) || i == get_rotation(330, self.rotation) || i == get_rotation(90, self.rotation) {
                triangle_points.append(&mut vec![[self.radius * t.cos() + self.pos.x as f64, self.radius * t.sin() + self.pos.y as f64]])
            }
        }
        return triangle_points;
    }

    pub fn add_rotation(&mut self, r:i32){
        self.rotation = get_rotation(self.rotation, r);
    }
}

fn get_rotation(start_r: i32,r:i32) -> i32 {
    let new_r = start_r + r.clamp(0, 360);
    if new_r < 0 {
        return 360 - new_r;
    }
    else if new_r > 360 {
        return new_r - 360;
    }
    else {
        return new_r;
    }
}