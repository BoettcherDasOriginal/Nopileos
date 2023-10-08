
#[derive(Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub comp_name: String,
    pub owner_code: String,
}

impl PlayerInfo {
    pub fn new(name: String, comp_name: String, owner_code: String) -> Self{
        Self { name: name, comp_name: comp_name, owner_code: owner_code }
    }
}