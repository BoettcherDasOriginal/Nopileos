use epaint::Color32;

#[derive(Clone)]
pub struct Sector{
    pub name: String,
    pub id: i32,
    pub star_name: String,
    pub star_radius: f64,
    pub star_color: Color32,
    pub border_color: Color32,
}

impl Sector{
    pub fn new(name: String,id: i32,star_name: String,star_radius: f64,star_color: Color32,border_color: Color32) -> Self {
        Self { name: name, id: id,star_name: star_name,star_radius: star_radius, star_color: star_color, border_color: border_color }
    }
}