use egui_extras::{TableBuilder, Column};

use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData, entities::{station::Station, entity::{EntitySettings, EntityType, EntityWareStorage, Entity}, command::EntityCommandHandler}, common::{vector2::Vector2,position::Position}};

pub struct StationInspector {
    shared_data: SharedGameData,
    selected_station: Station,
    selected_station_id: String,
}

impl Default for StationInspector {
    fn default() -> Self {
        Self {  
            shared_data: SharedGameData::new(),
            selected_station: Station::new(EntitySettings::new("None".to_string(), "None".to_string(), false, "None".to_string(), EntityType::Station, EntityCommandHandler::new(vec![])), crate::entities::station::StationType::Station, EntityWareStorage::new(vec![], 0.0), Position::new(0, Vector2::new(0.0, 0.0))),
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
            .default_width(300.0)
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
        let mut used_storage_space = 0.0;
        for w in self.selected_station.get_storage().storage {
            used_storage_space += w.0.storage_space * w.1 as f64;
        }
        
        egui::ScrollArea::vertical()
            .max_width(400.0)
            .max_height(500.0)
            .show(ui, |ui| {
                ui.heading("Info");
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Name:");
                        ui.label("ID:");
                        ui.label("Owner:");
                        ui.label("Sector:");
                        ui.label("Position:");
                    });
                    ui.vertical(|ui| {
                        ui.label(&self.selected_station.get_settings().name);
                        ui.label(&self.selected_station.get_settings().uid);
                        ui.label(&self.selected_station.get_settings().owner);
                        ui.label(&self.shared_data.sectors.get(self.selected_station.get_position().global_pos as usize).unwrap().name);
                        ui.label(&self.selected_station.get_position().local_pos.to_string());
                    });
                });

                ui.add_space(12.0);
                ui.heading("Storage");
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Capacity:");
                        ui.label("Used:");
                    });
                    ui.vertical(|ui| {
                        ui.label(&self.selected_station.get_storage().storage_space.to_string());
                        let used_in_p = used_storage_space / self.selected_station.get_storage().storage_space * 100.0;
                        ui.label(used_in_p.to_string() + "%");
                    });
                });

                ui.collapsing("Directory", |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(false)
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::initial(50.0).range(25.0..=100.0))
                        .column(Column::remainder())
                        .column(Column::remainder())
                        .column(Column::remainder())
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Ware");
                            });
                            header.col(|ui| {
                                ui.strong("Amount");
                            });
                            header.col(|ui| {
                                ui.strong("Type");
                            });
                            header.col(|ui| {
                                ui.strong("Space");
                            });
                        })
                        .body(|mut body| {
                            for w in self.selected_station.get_storage().storage {
                                let ware = w.0;
                                let amount = w.1;
        
                                body.row(20.0, |mut row| {
                                    row.col(|ui|{
                                        ui.label(ware.name);
                                    });
                                    row.col(|ui|{
                                        ui.label(amount.to_string());
                                    });
                                    row.col(|ui|{
                                        ui.label(ware.storage_type);
                                    });
                                    row.col(|ui|{
                                        ui.label((amount as f64 * ware.storage_space).to_string());
                                    });
                                });
                            }
                        }
                    );
                });
            }
        );
    }
}