use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData, player};

pub struct PlayerInfo {
    shared_data: SharedGameData,
    player_info: player::PlayerInfo,
    old_owner_name: String,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {  
            shared_data: SharedGameData::new(),
            player_info: player::PlayerInfo::new("Player".to_string(),"Player Inc.".to_string(),"player".to_string()),
            old_owner_name: "player".to_string(),
        }
    }
}

impl GuiWindow for PlayerInfo {
    fn name(&self) -> &'static str {
        "Player Info"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool,data: SharedGameData) -> SharedGameData{
        self.shared_data = data;
        self.player_info = self.shared_data.player_info.clone();
        self.old_owner_name = self.player_info.owner_code.clone();

        egui::Window::new(self.name())
            .default_width(500.0)
            .min_width(500.0)
            .open(open)
            .show(ctx, |ui| {
                use GuiView as _;
                self.ui(ui);
            });

        if self.shared_data.owner_codes.contains(&self.player_info.owner_code.clone()) {
            self.player_info.owner_code = self.old_owner_name.clone();
        }
        else if self.old_owner_name != self.player_info.owner_code {
            let mut i = 0;
            for mut sec_e in self.shared_data.entities.clone() {
            
                let mut j = 0;
                for mut e in sec_e.clone() {
                    if e.get_settings().owner == self.old_owner_name {
                        let mut set = e.get_settings();
                        set.owner = self.player_info.owner_code.clone();
                        e.set_settings(set);
                    }
                
                    sec_e[j] = e;
                
                    j += 1;
                }

                self.shared_data.entities[i] = sec_e;

                i += 1;
            }
        }

        self.shared_data.player_info = self.player_info.clone();
        return self.shared_data.clone();
    }

    fn killed(&mut self) {
        
    }
}

impl GuiView for PlayerInfo {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .max_width(400.0)
            .max_height(500.0)
            .show(ui, |ui| {
                ui.heading("Player");

                ui.add_space(2.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(2.0);
                        ui.label("Name:");
                        ui.add_space(4.0);
                        ui.label("Company Name:");
                        ui.add_space(4.0);
                        ui.label("Owner Code:");
                    });

                    ui.vertical(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut self.player_info.name).desired_width(200.0));
                        ui.add(egui::TextEdit::singleline(&mut self.player_info.comp_name).desired_width(200.0));
                        ui.add(egui::TextEdit::singleline(&mut self.player_info.owner_code).desired_width(200.0));
                    });
                });
            }
        );
    }
}