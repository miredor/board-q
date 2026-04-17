use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{Quest, Status};

const ACTIONS: &[&str] = &[
    "Deliver",
    "Rescue",
    "Repair",
    "Scout",
    "Guard",
    "Recover",
    "Map",
    "Clean",
    "Study",
    "Negotiate",
];

const OBJECTS: &[&str] = &[
    "the moonlit archive",
    "a lost backpack",
    "the ember bridge",
    "three stubborn mushrooms",
    "the village clock",
    "a whispering compass",
    "the dusty workbench",
    "the crystal pantry",
    "a runaway notebook",
    "the pantry dragon's ledger",
];

const BIOMES: &[&str] = &[
    "Forest",
    "Ruins",
    "Village",
    "Workshop",
    "Library",
    "Harbor",
    "Cavern",
    "Tower",
];

#[derive(Clone, Debug)]
struct Lcg {
    state: u64,
}

impl Lcg {
    fn seeded() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(42);

        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn index(&mut self, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        (self.next_u32() as usize) % len
    }

    fn range_u8(&mut self, start: u8, end_inclusive: u8) -> u8 {
        if start >= end_inclusive {
            return start;
        }
        let span = (end_inclusive - start + 1) as u32;
        start + (self.next_u32() % span) as u8
    }

    fn range_u32(&mut self, start: u32, end_inclusive: u32) -> u32 {
        if start >= end_inclusive {
            return start;
        }
        let span = end_inclusive - start + 1;
        start + (self.next_u32() % span)
    }
}

pub fn generate_many(count: usize) -> Vec<Quest> {
    let mut rng = Lcg::seeded();
    let mut quests = Vec::with_capacity(count);

    for _ in 0..count {
        quests.push(Quest {
            id: 0,
            title: format!(
                "{} {}",
                ACTIONS[rng.index(ACTIONS.len())],
                OBJECTS[rng.index(OBJECTS.len())]
            ),
            difficulty: rng.range_u8(1, 5),
            reward: rng.range_u32(20, 120),
            biome: BIOMES[rng.index(BIOMES.len())].to_string(),
            deadline_days: rng.range_u8(1, 14),
            status: Status::Todo,
        });
    }

    quests
}

pub fn make_custom_quest(id: u32, title: String) -> Quest {
    let title_len = title.chars().count() as u32;
    let difficulty = ((title_len % 5) + 1) as u8;
    let reward = 15 + title_len * 3;
    let biome = if title_len % 2 == 0 {
        "Village"
    } else {
        "Workshop"
    };
    let deadline_days = ((title_len % 7) + 1) as u8;

    Quest {
        id,
        title,
        difficulty,
        reward,
        biome: biome.to_string(),
        deadline_days,
        status: Status::Todo,
    }
}
