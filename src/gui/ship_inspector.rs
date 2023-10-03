use std::collections::BTreeMap;

use egui::RichText;

use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData, entities::{ship::Ship, entity::{EntitySettings, EntityType, EntityWareStorage, Entity}, command::EntityCommandHandler}, position::Position, common::vector2::Vector2};

pub struct ShipInspector {
    shared_data: SharedGameData,
    selected_ship: Ship,
    selected_ship_id: String,
}

impl Default for ShipInspector {
    fn default() -> Self {
        Self {  
            shared_data: SharedGameData::new(),
            selected_ship: Ship::new(EntitySettings::new("None".to_string(), "None".to_string(), false, "None".to_string(), EntityType::Station, EntityCommandHandler::new(vec![])), crate::entities::ship::ShipType::SFighter, EntityWareStorage::new(BTreeMap::new(), 0.0), Position::new(0, Vector2::new(0.0, 0.0))),
            selected_ship_id: "".to_string(),
        }
    }
}

impl GuiWindow for ShipInspector {
    fn name(&self) -> &'static str {
        "Ship Inspector"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool,data: SharedGameData) -> SharedGameData{
        self.shared_data = data;
        self.selected_ship_id = self.shared_data.selected_ship_id.clone();
        
        for sec_e in self.shared_data.entities.clone() {
            for mut e in sec_e.clone() {
                match e.get_settings().e_type {
                    EntityType::Ship => {
                        if e.get_settings().uid == self.selected_ship_id {
                            if let Some(s) = e.as_any().downcast_ref::<Ship>() {
                                self.selected_ship = s.clone();
                                break;
                            }
                        }
                    }
                    EntityType::Station => {
                        
                    }
                }
            }
        }

        egui::Window::new(self.name())
            .default_width(500.0)
            .min_width(500.0)
            .open(open)
            .show(ctx, |ui| {
                use GuiView as _;
                self.ui(ui);
            });

        return self.shared_data.clone();
    }

    fn killed(&mut self) {
        
    }
}

impl GuiView for ShipInspector {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Name:").strong());
                ui.label(RichText::new("ID:").strong());
                ui.label(RichText::new("Owner:").strong());
                ui.label(RichText::new("Sector:").strong());
                ui.label(RichText::new("Position:").strong());
            });
            ui.vertical(|ui| {
                ui.label(&self.selected_ship.get_settings().name);
                ui.label(&self.selected_ship.get_settings().uid);
                ui.label(&self.selected_ship.get_settings().owner);
                ui.label(&self.shared_data.sectors.get(self.selected_ship.get_position().global_pos as usize).unwrap().name);
                ui.label(&self.selected_ship.get_position().local_pos.to_string());
            });
        });
    }
}