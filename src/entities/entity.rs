use std::collections::BTreeMap;

use crate::wares::wares::Ware;
use crate::position::Position;

#[derive(Clone, Debug)]
pub enum EntityType {
    Ship,
    Station,
}

pub trait Entity: EntityClone {
    fn get_settings(&mut self) -> EntitySettings;
    fn get_storage(&mut self) -> EntityWareStorage;
    fn set_storage(&mut self,storage: EntityWareStorage) -> bool;
    fn get_position(&mut self) -> Position;
    fn set_position(&mut self,pos: Position);
}

#[derive(Clone, Debug)]
pub struct EntitySettings {
    pub name: String,
    pub uid: String, // Format: XCT-567, ABC-123
    pub is_static: bool,
    pub owner: String, // owner uid
    pub e_type: EntityType,
}

impl EntitySettings {
    pub fn new(name: String, uid: String, is_static: bool, owner: String, e_type: EntityType) -> Self {
        Self { name: name, uid: uid, is_static: is_static, owner: owner, e_type: e_type }
    }
}

#[derive(Clone, Debug)]
pub struct EntityWareStorage{
    storage: BTreeMap<Ware,i64>,
    storage_space: f64,
}

impl EntityWareStorage {
    pub fn new(storage: BTreeMap<Ware,i64>, storage_space: f64) -> Self{
        Self { storage: storage, storage_space: storage_space }
    }
}

//Cloning

trait EntityClone {
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