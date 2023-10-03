use open;
use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData};

pub struct About {}

impl Default for About {
    fn default() -> Self {
        Self {  }
    }
}

impl GuiWindow for About {
    fn name(&self) -> &'static str {
        "About"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool,data: SharedGameData) -> SharedGameData{
        egui::Window::new(self.name())
            .default_width(320.0)
            .open(open)
            .show(ctx, |ui| {
                use GuiView as _;
                self.ui(ui);
            });

        return data;
    }

    fn killed(&mut self) {
        
    }
}

impl GuiView for About {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Nopileos");
        ui.horizontal_wrapped(|ui| {
            ui.label("Nopileos is a science fiction economic simulation game that enables players to engage in trading, construct and manage space stations and fleets, and interact with various factions and their political dynamics. Begin as a humble trader and progress towards becoming the ruler of multiple sectors.");
            
        });

        ui.add_space(12.0);

        ui.heading("Early Access");
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("This game is written in ");
            if ui.hyperlink_to("Rust", "https://www.rust-lang.org/").clicked() { let _ = open::that("https://www.rust-lang.org/"); }
            ui.label(" and is currently in development!\nEnjoy ;)");
        });

        ui.add_space(12.0); // ui.separator();
        ui.heading("Links");
        links(ui);
    }
}

fn links(ui: &mut egui::Ui) {
    use egui::special_emojis::GITHUB;
    if ui.hyperlink_to(
        format!("{} source on GitHub", GITHUB),
        "https://github.com/BoettcherDasOriginal/Nopileos",
    ).clicked() {let _ = open::that("https://github.com/BoettcherDasOriginal/Nopileos");}
    //ui.hyperlink_to("documentation", "");
}