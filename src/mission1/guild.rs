pub struct Guild {
    name: String,
    total_members: i32,
}

impl Guild {
    pub fn new(name: String, total_members: i32) -> Guild {
        Guild {
            name: name,
            total_members: total_members,
        }
    }

    pub fn guild_info(&self) {
        println!("Guild Name: {}", self.name);
        println!("Total Members: {}", self.total_members);
    }
}