mod engine;
mod gui;

use crate::engine::game_window::run;
use crate::engine::gui_windows::GUIInterface;
use crate::engine::event_handler::EventHandler;

fn main() {
    let mut event_h = EventHandler::default();
    event_h.set_events(start_callback, update_callback, end_callback);
    pollster::block_on(run(event_h));
}

fn update_callback(guii: GUIInterface) -> GUIInterface{
    return guii;
}

fn start_callback(mut guii: GUIInterface) -> GUIInterface {
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

fn end_callback() {
    
}
