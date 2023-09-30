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
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Nopileos is a game written in ");
            if ui.hyperlink_to("Rust", "https://www.rust-lang.org/").clicked() { let _ = open::that("https://www.rust-lang.org/"); }
            ui.label(" .");
        });

        ui.add_space(12.0);

        ui.heading("Early Access");
        ui.label("This game is currently in development!");

        ui.add_space(12.0); // ui.separator();
        ui.heading("Links");
        links(ui);
    }
}

fn links(ui: &mut egui::Ui) {
    use egui::special_emojis::GITHUB;
    if ui.hyperlink_to(
        format!("{} source on GitHub", GITHUB),
        "https://github.com/BoettcherDasOriginal/",
    ).clicked() {let _ = open::that("https://github.com/BoettcherDasOriginal/");}
    //ui.hyperlink_to("documentation", "");
}