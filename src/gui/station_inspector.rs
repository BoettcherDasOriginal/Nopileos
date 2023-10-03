use std::collections::BTreeMap;

use egui::RichText;

use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData, entities::{station::Station, entity::{EntitySettings, EntityType, EntityWareStorage, Entity}, command::EntityCommandHandler}, position::Position, common::vector2::Vector2};

pub struct StationInspector {
    shared_data: SharedGameData,
    selected_station: Station,
    selected_station_id: String,
}

impl Default for StationInspector {
    fn default() -> Self {
        Self {  
            shared_data: SharedGameData::new(),
            selected_station: Station::new(EntitySettings::new("None".to_string(), "None".to_string(), false, "None".to_string(), EntityType::Station, EntityCommandHandler::new(vec![])), crate::entities::station::StationType::Station, EntityWareStorage::new(BTreeMap::new(), 0.0), Position::new(0, Vector2::new(0.0, 0.0))),
            selected_station_id: "".to_string(),
        }
    }
}

impl GuiWindow for StationInspector {
    fn name(&self) -> &'static str {
        "Station Inspector"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool,data: SharedGameData) -> SharedGameData{
        self.shared_data = data;
        self.selected_station_id = self.shared_data.selected_station_id.clone();
        
        for sec_e in self.shared_data.entities.clone() {
            for mut e in sec_e.clone() {
                match e.get_settings().e_type {
                    EntityType::Ship => {
                        
                    }
                    EntityType::Station => {
                        if e.get_settings().uid == self.selected_station_id {
                            if let Some(s) = e.as_any().downcast_ref::<Station>() {
                                self.selected_station = s.clone();
                                break;
                            }
                        }
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

impl GuiView for StationInspector {
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
                ui.label(&self.selected_station.get_settings().name);
                ui.label(&self.selected_station.get_settings().uid);
                ui.label(&self.selected_station.get_settings().owner);
                ui.label(&self.shared_data.sectors.get(self.selected_station.get_position().global_pos as usize).unwrap().name);
                ui.label(&self.selected_station.get_position().local_pos.to_string());
            });
        });
    }
}