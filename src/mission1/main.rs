mod adventurer;

fn main() {
    let player1 = adventurer::Adventurer::new(String::from("minho"), 100, 50);
    player1.introduce();
}
