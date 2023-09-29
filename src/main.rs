mod engine;
mod gui;

use crate::engine::game_window::run;
use crate::engine::gui_windows::GUIInterface;
use crate::engine::game_window::GameWindow;

fn main() {
    pollster::block_on(run(Nopileos::default()));
}

struct Nopileos {

}

impl Default for Nopileos {
    fn default() -> Self {
        Self {  }
    }
}

impl GameWindow for Nopileos {
    fn start(&mut self, mut guii: GUIInterface) -> GUIInterface {
        guii.add_guis.append(&mut vec![
            //MenuBars, etc -> must be before windows so they can't go on top
            Box::new(gui::menu_bar::MenuBar::default()),
            //Windows
            Box::new(gui::about::About::default()),
        ]);
        guii.open_guis.insert("menu_bar".to_string());
        guii.open_guis.insert("About".to_string());
    
        return guii;
    }

    fn update(&mut self, guii: GUIInterface) -> GUIInterface{
        return guii;
    }
    
    fn end(&mut self) {
        
    }
}
