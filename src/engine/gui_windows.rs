use egui::Context;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use egui::FontData;
use std::collections::BTreeSet;
use egui::epaint::FontFamily;

use crate::SharedGameData;

pub trait GuiWindow {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool,data: SharedGameData) -> SharedGameData;

    // Kill callback
    fn killed(&mut self);
}

pub trait GuiView {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub struct GuiWindows {
    pub guis: Vec<Box<dyn GuiWindow>>,
    pub open: BTreeSet<String>,
}

impl Default for GuiWindows {
    fn default() -> Self {
        Self::set_guis(vec![

        ])
    }
}

impl GuiWindows {
    pub fn set_guis(guis: Vec<Box<dyn GuiWindow>>) -> Self{
        let open = BTreeSet::new();

        Self { guis, open }
    }

    pub fn windows(&mut self, ctx: &Context,mut data: SharedGameData) -> SharedGameData {
        let Self { guis, open } = self;
        for gui in guis {
            let mut is_open = open.contains(gui.name());
            data = gui.show(ctx, &mut is_open,data);
            set_open(open, gui.name(), is_open);
        }

        return data;
    }

    pub fn ui(&mut self, ctx: &Context,guii: GUIInterface,mut data: SharedGameData) -> SharedGameData{
        self.style(ctx);
        data = self.windows(ctx,data);
        self.interface(guii);

        return data;
    }

    pub fn interface(&mut self,mut guii: GUIInterface){
        self.guis.append(&mut guii.add_guis);
        
        for str in guii.open_guis{
            self.open.insert(str);
        }

        for str in guii.close_guis{
            self.open.remove(&str);
        }

        for rm in guii.remove_guis{
            let i = self.guis.iter().position(|x| *x.name() == rm).unwrap();
            self.guis[i].killed();
            self.open.remove(&rm);
            self.guis.remove(i);
        }
    }

    pub fn style(&mut self,ctx: &Context){
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert("droid_sans".to_owned(),
        FontData::from_static(include_bytes!("../fonts/DroidSans.ttf")));
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "droid_sans".to_owned());

        fonts.font_data.insert("inconsolata".to_owned(),
        FontData::from_static(include_bytes!("../fonts/Inconsolata-VF.ttf")));
        fonts.families.insert(egui::FontFamily::Name("inconsolata".into()), vec!["inconsolata".to_owned()]);
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().push("inconsolata".to_owned());

        ctx.set_fonts(fonts);
        
        let mut style = (*ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
        (Heading, FontId::new(18.0, Proportional)),
        (Body, FontId::new(12.5, Proportional)),
        (Monospace, FontId::new(12.0, Proportional)),
        (Button, FontId::new(12.5, Proportional)),
        (Small, FontId::new(9.0, Proportional)),
        ].into();

        // Mutate global style with above changes
        ctx.set_style(style);
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

pub struct GUIInterface{
    pub add_guis: Vec<Box<dyn GuiWindow>>,
    pub remove_guis: BTreeSet<String>,
    pub close_guis: BTreeSet<String>,
    pub open_guis: BTreeSet<String>,
}

impl Default for GUIInterface {
    fn default() -> Self {
        let open = BTreeSet::new();
        let close = BTreeSet::new();
        let remove = BTreeSet::new();
        let add = vec![];

        Self { add_guis: add, remove_guis: remove, close_guis: close, open_guis: open }
    }
}

impl GUIInterface {
    /* --- Dead Code ---
    pub fn set_guis(aguis: Vec<Box<dyn GuiWindow>>, rguis: BTreeSet<String>, oguis: BTreeSet<String>) -> Self{
        Self { add_guis: aguis, remove_guis: rguis, open_guis: oguis }
    }*/
}