#[derive(Clone, Debug)]
pub struct Ware {
    pub name: String,
    pub storage_type: String,
    pub storage_space: f64,
    pub normal_price: f64,
}

impl Ware {
    pub fn new(name: String,storage_type: String, storage_space: f64, normal_price: f64) -> Self{
        Self { name: name, storage_type: storage_type, storage_space: storage_space, normal_price: normal_price }
    }
}