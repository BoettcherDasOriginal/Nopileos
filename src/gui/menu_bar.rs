use egui::Modifiers;
use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData};
use std::process;

pub struct MenuBar {}

impl Default for MenuBar {
    fn default() -> Self {
        Self {  }
    }
}

impl GuiWindow for MenuBar {
    fn name(&self) -> &'static str {
        "menu_bar"
    }

    fn show(&mut self, ctx: &egui::Context, _open: &mut bool,data: SharedGameData) -> SharedGameData{
        egui::TopBottomPanel::top("menu_bar")
        .min_height(25.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("NOPILEOS");

                egui::menu::bar(ui, |ui| {
                    use GuiView as _;
                    self.ui(ui);
                });


            });

            ui.label(format!("delta_time:{}",data.delta_time))
        });

        return data;
    }

    fn killed(&mut self) {
        
    }
}

impl GuiView for MenuBar {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let organize_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
        let reset_shortcut = egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

        // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
        // or else they would only be checked if the "File" menu was actually open!

        if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
        }

        if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
        }

        ui.horizontal(|ui| {
            ui.menu_button("EGUI", |ui| {

                if ui
                    .add(
                        egui::Button::new("Organize Windows")
                            .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
                    )
                    .clicked()
                {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                    ui.close_menu();
                }
    
                if ui
                    .add(
                        egui::Button::new("Reset egui memory")
                            .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
                    )
                    .on_hover_text("Forget scroll, positions, sizes etc")
                    .clicked()
                {
                    ui.ctx().memory_mut(|mem| *mem = Default::default());
                    ui.close_menu();
                }
            });
            ui.menu_button("Exit", |ui| {
                if ui.button("Exit").clicked(){
                    process::exit(0);
                }
            });
        });

        
    }
}
