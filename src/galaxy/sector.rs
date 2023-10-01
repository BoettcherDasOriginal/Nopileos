use crate::entities::entity::Entity;
use epaint::Color32;

#[derive(Clone)]
pub struct Sector{
    pub name: String,
    pub star_name: String,
    pub star_radius: f64,
    pub star_color: Color32,
    pub border_color: Color32,
    pub entities: Vec<Box<dyn Entity>>,
}

impl Sector{
    pub fn new(name: String,star_name: String,star_radius: f64,star_color: Color32,border_color: Color32,entities: Vec<Box<dyn Entity>>) -> Self {
        Self { name: name, star_name: star_name,star_radius: star_radius, star_color: star_color, border_color: border_color, entities: entities }
    }
}