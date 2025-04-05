pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

#[derive(Debug)]
pub enum ProductCategory {
    Ladder, //public by default
    Hammer,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    category: ProductCategory, //private by default
    quantity: u32,
}

impl Item {
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        Self {
            name,
            category,
            quantity
        }
    }
}

pub fn take_to_manager () {
    println!("Hey, {MANAGER}, how's your coffee?");
}

