#[path = "../src/models.rs"]
mod models;
#[path = "../src/generator.rs"]
mod generator;

use models::{Quest, Status};

#[test]
fn quest_roundtrip_record_works() {
    let original = Quest {
        id: 7,
        title: "Repair the village clock".to_string(),
        difficulty: 3,
        reward: 55,
        biome: "Village".to_string(),
        deadline_days: 4,
        status: Status::Todo,
    };

    let line = original.to_record();
    let restored = Quest::from_record(&line).expect("record should parse");

    assert_eq!(original, restored);
}

#[test]
fn custom_quest_uses_given_id_and_title() {
    let quest = generator::make_custom_quest(99, "Sort all cables".to_string());

    assert_eq!(quest.id, 99);
    assert_eq!(quest.title, "Sort all cables");
    assert_eq!(quest.status, Status::Todo);
}
