pub struct Adventurer {
    name: String,
    life: i32,
    gold: i32,
}

impl Adventurer {
    pub fn new(name: String, life: i32, gold: i32) -> Adventurer {
        Adventurer {
            name: name,
            life: life,
            gold: gold,
        }
    }

    pub fn introduce(&self) {
        println!("hello my name is {}", self.name);
        println!("my life is {}", self.life);
        println!("have {} gold", self.gold);
    }
}
