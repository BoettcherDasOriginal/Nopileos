use crate::common::vector2::Vector2;

#[derive(Clone, Debug)]
pub struct Position{
    global_pos: String,
    local_pos: Vector2,
}

impl Position {
    pub fn new(global_pos: String,local_pos: Vector2) -> Self{
        Self { global_pos: global_pos, local_pos: local_pos }
    }
}