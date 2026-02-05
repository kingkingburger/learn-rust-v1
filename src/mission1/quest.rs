use crate::item::Item;

pub struct Quest {
    pub name: String,
    pub level: i32,
    pub reward: Item,
}
