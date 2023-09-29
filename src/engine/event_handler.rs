use super::gui_windows::GUIInterface;


type Start = fn(guii: GUIInterface) -> GUIInterface;
type Update = fn(guii: GUIInterface) -> GUIInterface;
type End = fn();

pub struct EventHandler {
    pub first_frame: bool,
    pub start_event: Start,
    pub update_event: Update,
    pub end_event: End,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self { first_frame: true,start_event: default_start, update_event: default_update, end_event: default_end }
    }
}

impl EventHandler {
    pub fn set_events(&mut self,start_e: Start, update_e: Update, end_e: End) {
        self.start_event = start_e;
        self.update_event = update_e;
        self.end_event = end_e;
    }

    pub fn start(&mut self,guii: GUIInterface) -> GUIInterface{
        let Self { first_frame: _, start_event, update_event: _, end_event: _} = self;
        return start_event(guii);
    }
    pub fn update(&mut self,guii: GUIInterface) -> GUIInterface{
        let Self { first_frame: _, start_event: _, update_event, end_event: _} = self;
        return update_event(guii);
    }
    pub fn end(&mut self) {
        let Self { first_frame: _, start_event: _, update_event: _, end_event} = self;
        return end_event();
    }
}

fn default_start(guii: GUIInterface) -> GUIInterface {
    return guii;
}

fn default_update(guii: GUIInterface) -> GUIInterface {
    return guii;
}

fn default_end() {
    
}