use egui_extras::{TableBuilder, Column};

use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData, entities::{ship::Ship, entity::{EntitySettings, EntityType, EntityWareStorage, Entity}, command::EntityCommandHandler}, common::{vector2::Vector2,position::Position}};

pub struct ShipCommander {
    shared_data: SharedGameData,
    selected_ship: Ship,
    selected_ship_id: String,
}

impl Default for ShipCommander {
    fn default() -> Self {
        Self {  
            shared_data: SharedGameData::new(),
            selected_ship: Ship::new(EntitySettings::new("None".to_string(), "None".to_string(), false, "None".to_string(), EntityType::Station, EntityCommandHandler::new(vec![])), crate::entities::ship::ShipType::S, EntityWareStorage::new(vec![], 0.0), Position::new(0, Vector2::new(0.0, 0.0)),0.0),
            selected_ship_id: "".to_string(),
        }
    }
}

impl GuiWindow for ShipCommander {
    fn name(&self) -> &'static str {
        "Ship Commander"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool,data: SharedGameData) -> SharedGameData{
        self.shared_data = data;
        
        for sec_e in self.shared_data.entities.clone() {
            for mut e in sec_e.clone() {
                match e.get_settings().e_type {
                    EntityType::Ship => {
                        if e.get_settings().uid == self.shared_data.selected_ship_id && e.get_settings().owner == self.shared_data.player_info.owner_code {
                            if let Some(s) = e.as_any().downcast_ref::<Ship>() {
                                self.selected_ship = s.clone();
                                self.selected_ship_id = self.selected_ship.get_settings().uid.clone();
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

        //update settings

        let mut i = 0;
            for mut sec_e in self.shared_data.entities.clone() {
            
                let mut j = 0;
                for mut e in sec_e.clone() {
                    if e.get_settings().uid == self.selected_ship.get_settings().uid {
                        e = Box::new(self.selected_ship.clone());
                    }
                
                    sec_e[j] = e;
                
                    j += 1;
                }

                self.shared_data.entities[i] = sec_e;

                i += 1;
            }

        return self.shared_data.clone();
    }

    fn killed(&mut self) {
        
    }
}

impl GuiView for ShipCommander {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let mut used_storage_space = 0.0;
        for w in self.selected_ship.get_storage().storage {
            used_storage_space += w.0.storage_space * w.1 as f64;
        }

        egui::ScrollArea::vertical()
            .max_width(400.0)
            .max_height(500.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("")
                    .width(200.0)
                    .selected_text(format!("{} ({})",self.selected_ship.get_settings().name,self.selected_ship.get_settings().uid))
                    .show_ui(ui, |ui| {
                        for i in self.shared_data.entities.clone() {
                            for j in i {
                                if let Some(s) = j.as_any().downcast_ref::<Ship>() {
                                    let mut ship = s.clone();

                                    if ship.get_settings().owner != self.shared_data.player_info.owner_code {
                                        continue;
                                    }

                                    ui.selectable_value(&mut self.selected_ship_id, ship.get_settings().uid, format!("{} ({})",ship.get_settings().name,ship.get_settings().uid));
                                    self.shared_data.selected_ship_id = self.selected_ship_id.clone();
                                }
                            }
                        }
                    });
                });

                let mut new_ship_name = self.selected_ship.get_settings().name.clone();

                ui.heading("Info");
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(2.0);
                        ui.label("Name:");
                        ui.add_space(2.0);
                        ui.label("ID:");
                        ui.label("Owner:");
                        ui.label("Sector:");
                        ui.label("Position:");
                    });
                    ui.vertical(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut new_ship_name).desired_width(200.0));
                        ui.label(&self.selected_ship.get_settings().uid);
                        ui.label(&self.selected_ship.get_settings().owner);
                        ui.label(&self.shared_data.sectors.get(self.selected_ship.get_position().global_pos as usize).unwrap().name);
                        ui.label(&self.selected_ship.get_position().local_pos.to_string());
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
                        ui.label(&self.selected_ship.get_storage().storage_space.to_string());
                        let used_in_p = used_storage_space / self.selected_ship.get_storage().storage_space * 100.0;
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
                            for w in self.selected_ship.get_storage().storage {
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

                //update ship setttings
                let mut set = self.selected_ship.get_settings();
                set.name = new_ship_name;

                self.selected_ship.set_settings(set);
            }
        );
    }
}