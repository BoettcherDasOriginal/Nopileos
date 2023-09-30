mod engine;
mod gui;
mod entities;
mod wares;
mod common;
mod position;
mod galaxy;

use egui::Context;

use crate::engine::game_window::run;
use crate::engine::gui_windows::GUIInterface;
use crate::engine::game_window::GameWindow;
use crate::engine::gui_windows::GuiWindows;

fn main() {
    pollster::block_on(run(Nopileos::default()));
}

#[derive(Clone, Debug)]
pub struct SharedGameData {
    delta_time: f64,
}

impl SharedGameData {
    pub fn new() -> Self{
        Self { delta_time: 0.0 }
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
