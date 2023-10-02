use crate::common::vector2::Vector2;
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
    fn set_settings(&mut self,settings: EntitySettings) {
        self.settings = settings;
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
    pub fn new(settings: EntitySettings,ship_type: ShipType,storage: EntityWareStorage,position: Position) -> Self{
        Self { settings: settings, ship_type: ship_type, storage: storage, position: position }
    }

    pub fn get_ship_type(&mut self) -> ShipType{
        return self.ship_type.clone();
    }

    pub fn move_ship_local(&mut self, v: f64,target: Vector2,delta_time: f64) {
        let pos = self.get_position();
        let t = target - pos.local_pos;

        let new_pos = Vector2::new(
            self.get_position().local_pos.x + ((v * (1.0 * (t.x / t.y))) * delta_time * t.x.clamp(-1.0, 1.0)), 
            self.get_position().local_pos.y + ((v * (1.0 * (t.y / t.x))) * delta_time * t.y.clamp(-1.0, 1.0))
        );

        self.set_position(Position::new(pos.global_pos, new_pos));
    }
}