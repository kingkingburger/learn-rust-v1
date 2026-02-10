use crate::item::Item;

pub struct Quest {
    pub name: String,
    pub level: i32,
    pub reward: Item,
}

impl Quest {
    pub fn new(name: &str, level: i32, reward: Item) -> Self {
        Quest {
            name: name.to_string(),
            level,
            reward,
        }
    }

    pub fn show_info(&self) {
        println!("Quest Name: {}", self.name);
        println!("Quest Level: {}", self.level);
        println!("Reward Item: {}", self.reward.name);
    }
}