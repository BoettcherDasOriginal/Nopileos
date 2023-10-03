use crate::common::toany::ToAny;
use crate::common::vector2::Vector2;
use crate::wares::wares::Ware;
use crate::position::Position;

use super::command::EntityCommandHandler;

#[derive(Clone, Debug)]
pub enum EntityType {
    Ship,
    Station,
}

pub trait Entity: EntityClone + ToAny {
    fn get_settings(&mut self) -> EntitySettings;
    fn set_settings(&mut self,settings: EntitySettings);
    fn get_storage(&mut self) -> EntityWareStorage;
    fn set_storage(&mut self,storage: EntityWareStorage) -> bool;
    fn get_position(&mut self) -> Position;
    fn set_position(&mut self,pos: Position);

    fn is_mouse_hovered(&mut self,mouse_pos: Vector2,radius: f64) -> bool{
        let min = self.get_position().local_pos - Vector2::new(radius, radius);
        let max = self.get_position().local_pos + Vector2::new(radius, radius);

        min.x < mouse_pos.x && max.x > mouse_pos.x &&
        min.y < mouse_pos.y && max.y > mouse_pos.y
    }
}

#[derive(Clone, Debug)]
pub struct EntitySettings {
    pub name: String,
    pub uid: String, // Format: XCT-567, ABC-123
    pub is_static: bool,
    pub owner: String, // owner uid
    pub e_type: EntityType,
    pub e_handler: EntityCommandHandler,
}

impl EntitySettings {
    pub fn new(name: String, uid: String, is_static: bool, owner: String, e_type: EntityType, e_handler: EntityCommandHandler) -> Self {
        Self { name: name, uid: uid, is_static: is_static, owner: owner, e_type: e_type, e_handler: e_handler }
    }
}

#[derive(Clone, Debug)]
pub struct EntityWareStorage{
    pub storage: Vec<(Ware,i64)>,
    pub storage_space: f64,
}

impl EntityWareStorage {
    pub fn new(storage: Vec<(Ware,i64)>, storage_space: f64) -> Self{
        Self { storage: storage, storage_space: storage_space }
    }
}

//Cloning

pub trait EntityClone {
    fn clone_box(&self) -> Box<dyn Entity>;
}

impl<T> EntityClone for T
where
    T: 'static + Entity + Clone
{
    fn clone_box(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Entity> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}