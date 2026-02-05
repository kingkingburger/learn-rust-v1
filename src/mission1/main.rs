mod adventurer;
mod item;
mod quest;

fn main() {
    let player1 = adventurer::Adventurer::new(String::from("minho"), 100, 50);
    player1.introduce();

    let reward_item = item::Item {
        name: String::from("일일퀘스트"),
        effect: String::from("일일퀘스트"),
    };

    let quest = quest::Quest {
        name: String::from("일일퀘스트"),
        level: 0,
        reward: reward_item,
    };
}
