mod engine;
mod gui;
mod entities;
mod wares;
mod common;
mod position;
mod galaxy;

use std::collections::BTreeMap;

use egui::{Context,Color32};
use entities::entity::Entity;

use crate::engine::game_window::run;
use crate::engine::gui_windows::GUIInterface;
use crate::engine::game_window::GameWindow;
use crate::engine::gui_windows::GuiWindows;

use crate::{entities::entity::{EntityType, EntitySettings, EntityWareStorage},galaxy::sector::Sector, position::Position, common::vector2::Vector2};

fn main() {
    pollster::block_on(run(Nopileos::default()));
}

#[derive(Clone)]
pub struct SharedGameData {
    delta_time: f64,
    sectors: Vec<Sector>,
    entities: Vec<Box<dyn Entity>>,
}

impl SharedGameData {
    pub fn new() -> Self{
        Self { 
            delta_time: 0.0,
            sectors: vec![],
            entities: vec![]
        }
    }
}

struct Nopileos {
    gui_app: GuiWindows,
    shared_game_data: SharedGameData,
}

impl Default for Nopileos {
    fn default() -> Self {
        Self { gui_app: GuiWindows::default(), shared_game_data: SharedGameData::new() } 
    }
}

impl GameWindow for Nopileos {
    fn delta_time(&mut self) -> f64 {
        return self.shared_game_data.delta_time;
    }
    fn set_delta_time(&mut self,dt: f64) {
        self.shared_game_data.delta_time = dt;
    }

    fn gui_app(&mut self,ctx: &Context,guii: GUIInterface) {
        self.shared_game_data = self.gui_app.ui(ctx, guii,self.shared_game_data.clone());
    }

    fn start(&mut self, mut guii: GUIInterface) -> GUIInterface {
        let ship = crate::entities::ship::Ship::new(EntitySettings::new("Gox".to_string(), "HXI-739".to_string(), false, "owner".to_string(), EntityType::Ship), crate::entities::ship::ShipType::SFighter, EntityWareStorage::new(BTreeMap::new(), 100.0), Position::new(0, Vector2::new(87.3345, 33.5)));
        let station = crate::entities::station::Station::new(EntitySettings::new("Handelsstation".to_string(), "TLO-101".to_string(), false, "owner".to_string(), EntityType::Station), crate::entities::station::StationType::Station, EntityWareStorage::new(BTreeMap::new(), 100.0), Position::new(0, Vector2::new(100.0, 200.0)));
        self.shared_game_data.entities.append(&mut vec![Box::new(ship),Box::new(station)]);

        self.shared_game_data.sectors.append(& mut vec![Sector::new("Isaeuma Tlo'nep".to_string(),0, "Isaeuma IV".to_string(), 5.0, Color32::LIGHT_YELLOW, Color32::LIGHT_BLUE)]);
        self.shared_game_data.sectors.append(& mut vec![Sector::new("Isaeuma".to_string(),1, "Isaeuma I".to_string(), 15.0, Color32::LIGHT_RED, Color32::LIGHT_BLUE)]);

        guii.add_guis.append(&mut vec![
            //MenuBars, etc -> must be before windows so they can't go on top
            Box::new(gui::menu_bar::MenuBar::default()),
            //Windows
            Box::new(gui::about::About::default()),
            Box::new(gui::sector_map::SectorMap::default()),
        ]);
        guii.open_guis.insert("menu_bar".to_string());
        guii.open_guis.insert("About".to_string());
        guii.open_guis.insert("Sector Map".to_string());
        return guii;
    }

    fn update(&mut self, guii: GUIInterface) -> GUIInterface{
        return guii;
    }
    
    fn end(&mut self) {
        
    }
}
