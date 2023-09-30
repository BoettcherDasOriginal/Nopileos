#[derive(Clone, Debug)]
pub struct Ware {
    name: String,
    storage_type: String,
    storage_space: i64,
    normal_price: f64,
}

impl Ware {
    pub fn new(name: String,storage_type: String, storage_space: i64, normal_price: f64) -> Self{
        Self { name: name, storage_type: storage_type, storage_space: storage_space, normal_price: normal_price }
    }
}