mod engine;
mod gui;
mod entities;
mod wares;
mod common;
mod position;
mod galaxy;

use std::collections::{BTreeMap, BTreeSet};

use egui::{Context,Color32};
use entities::command::{EntityCommandHandler, EntityCommand};
use entities::entity::Entity;
use entities::ship::Ship;

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
    gui_wins: BTreeMap<String,bool>,
    gui_open_request: BTreeSet<String>,
    gui_close_request: BTreeSet<String>,
    selected_station_id: String,
    selected_ship_id: String,
    sectors: Vec<Sector>,
    entities: Vec<Vec<Box<dyn Entity>>>, // entities[sector_id] -> Vec<Box<dyn Entity>> in the given sector
}

impl SharedGameData {
    pub fn new() -> Self{
        Self { 
            delta_time: 0.0,
            gui_wins: BTreeMap::new(),
            gui_open_request: BTreeSet::new(),
            gui_close_request: BTreeSet::new(),
            selected_station_id: "None".to_string(),
            selected_ship_id: "None".to_string(),
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
        let ship_cmds = vec![
            EntityCommand::FlyToPos(Position::new(0, Vector2::new(-100.0, 10.0))),
            EntityCommand::FlyToPos(Position::new(0, Vector2::new(200.0, 200.0))),
            EntityCommand::FlyToPos(Position::new(0, Vector2::new(0.0, 100.0))),
        ];

        let mut ship = crate::entities::ship::Ship::new(EntitySettings::new("Gox".to_string(), "HXI-739".to_string(), false, "Civil".to_string(), EntityType::Ship, EntityCommandHandler::new(ship_cmds)), crate::entities::ship::ShipType::SFighter, EntityWareStorage::new(BTreeMap::new(), 100.0), Position::new(0, Vector2::new(0.0, 100.0)), 10.0);
        let station = crate::entities::station::Station::new(EntitySettings::new("Handelsstation".to_string(), "TLO-101".to_string(), false, "Civil".to_string(), EntityType::Station, EntityCommandHandler::new(vec![])), crate::entities::station::StationType::Station, EntityWareStorage::new(BTreeMap::new(), 100.0), Position::new(0, Vector2::new(100.0, 200.0)));
        let mut ship_set = ship.get_settings();
        ship_set.e_handler.get_current_command();
        ship.set_settings(ship_set);
        self.shared_game_data.entities.append(&mut vec![
            vec![Box::new(ship),Box::new(station)],
            vec![],
        ]);

        self.shared_game_data.sectors.append(& mut vec![Sector::new("Isaeuma Tlo'nep".to_string(),0, "Isaeuma IV".to_string(), 5.0, Color32::LIGHT_YELLOW, Color32::LIGHT_BLUE)]);
        self.shared_game_data.sectors.append(& mut vec![Sector::new("Isaeuma".to_string(),1, "Isaeuma I".to_string(), 15.0, Color32::LIGHT_RED, Color32::LIGHT_BLUE)]);

        guii.add_guis.append(&mut vec![
            //MenuBars, etc -> must be before windows so they can't go on top
            Box::new(gui::menu_bar::MenuBar::default()),
            //Windows
            Box::new(gui::about::About::default()),
            Box::new(gui::sector_map::SectorMap::default()),
            Box::new(gui::station_inspector::StationInspector::default()),
            Box::new(gui::ship_inspector::ShipInspector::default()),
        ]);
        guii.open_guis.insert("menu_bar".to_string());
        guii.open_guis.insert("About".to_string());
        guii.open_guis.insert("Sector Map".to_string());

        return guii;
    }

    fn update(&mut self, mut guii: GUIInterface) -> GUIInterface {

        self.shared_game_data.gui_wins = BTreeMap::new();
        for gui in &self.gui_app.guis {
            let is_open = self.gui_app.open.contains(gui.name());
            self.shared_game_data.gui_wins.insert(gui.name().to_string(), is_open);
        }

        for gui in self.shared_game_data.gui_open_request.clone() {
            let name = gui;
            guii.open_guis.insert(name.to_string());
            self.shared_game_data.gui_open_request.remove(&name);
        }

        for gui in self.shared_game_data.gui_close_request.clone() {
            let name = gui;
            guii.close_guis.insert(name.to_string());
            self.shared_game_data.gui_close_request.remove(&name);
        }

        let mut i = 0;
        for mut sec_e in self.shared_game_data.entities.clone() {

            let mut j = 0;
            for mut e in sec_e.clone() {
                match e.get_settings().e_type {
                    EntityType::Ship => {
                        match e.get_settings().e_handler.current_command {
                            EntityCommand::FlyToPos(mut pos) => {
                                if pos.global_pos == e.get_position().global_pos {
                                    if let Some(s) = e.as_any().downcast_ref::<Ship>() {
                                        let mut ship = s.clone();

                                        if !pos.local_pos.in_quad_radius(ship.get_position().local_pos, 0.1) {
                                            ship.move_ship_local(ship.max_speed, pos.local_pos, self.shared_game_data.delta_time);
                                        }
                                        else {
                                            ship.set_position(Position::new(pos.global_pos, pos.local_pos));
                                            let mut h = ship.get_settings();
                                            h.e_handler.get_next_command();
                                            h.e_handler.get_current_command();
                                            ship.set_settings(h);
                                        }
                                        e = Box::new(ship);
                                    }
                                }
                            }
                            EntityCommand::Null => {
                                
                            }
                        }
                    }
                    EntityType::Station => {

                    }
                }

                sec_e[j] = e;

                j += 1;
            }

            self.shared_game_data.entities[i] = sec_e;

            i += 1;
        }

        return guii;
    }
    
    fn end(&mut self) {
        
    }
}
