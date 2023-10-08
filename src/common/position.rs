use crate::common::vector2::Vector2;

#[derive(Clone, Debug)]
pub struct Position{
    pub global_pos: i32,
    pub local_pos: Vector2,
}

impl Position {
    pub fn new(global_pos: i32,local_pos: Vector2) -> Self{
        Self { global_pos: global_pos, local_pos: local_pos }
    }
}