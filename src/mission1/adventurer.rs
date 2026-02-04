pub struct Adventurer {
    name: String,
    life: i32,
    gold: i32,
}

impl Adventurer {
    pub fn introduce(&self) {
        println!("hello my name is {}", self.name);
        println!("my life is {}", self.life);
        println!("have {} gold" {self.gold});
    }
}
