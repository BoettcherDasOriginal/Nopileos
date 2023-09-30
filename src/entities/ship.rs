use crate::entities::entity::{Entity,EntityWareStorage,EntitySettings};
use crate::position::Position;

#[derive(Clone, Debug)]
pub enum ShipType {
    SFreighter,
    SFighter,
    MFreighter,
    MFighter,
}

#[derive(Clone, Debug)]
pub struct Ship{
    settings: EntitySettings,
    ship_type: ShipType,
    storage: EntityWareStorage,
    position: Position,
}

impl Entity for Ship {
    fn get_settings(&mut self) -> EntitySettings {
        return self.settings.clone();
    }

    fn get_storage(&mut self) -> EntityWareStorage {
        return self.storage.clone();
    }

    fn set_storage(&mut self,storage: EntityWareStorage) -> bool {
        self.storage = storage;
        return true;
    }

    fn get_position(&mut self) -> Position{
        return self.position.clone();
    }

    fn set_position(&mut self,pos: Position) {
        self.position = pos;
    }
}

impl Ship {
    fn new(settings: EntitySettings,ship_type: ShipType,storage: EntityWareStorage,position: Position) -> Self{
        Self { settings: settings, ship_type: ship_type, storage: storage, position: position }
    }

    fn get_ship_type(&mut self) -> ShipType{
        return self.ship_type.clone();
    }
}