use crate::entities::entity::{Entity,EntityWareStorage,EntitySettings};
use crate::common::position::Position;

#[derive(Clone, Debug)]
pub enum StationType {
    Station, //Default, can do nothing
}

#[derive(Clone, Debug)]
pub struct Station{
    settings: EntitySettings,
    pub station_type: StationType,
    storage: EntityWareStorage,
    position: Position,
}

impl Entity for Station {
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

impl Station {
    pub fn new(settings: EntitySettings,station_type: StationType,storage: EntityWareStorage,position: Position) -> Self{
        Self { settings: settings, station_type: station_type, storage: storage, position: position }
    }
}