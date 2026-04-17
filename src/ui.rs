use crate::models::{Quest, Status};

pub fn print_help() {
    println!("quest-board — a tiny fantasy CLI\n");
    println!("Commands:");
    println!("  generate [count]   Generate random quests");
    println!("  add <title>        Add a custom quest");
    println!("  list               Show all quests");
    println!("  done <id>          Mark quest as completed");
    println!("  stats              Show progress summary");
    println!("  help               Show this help message");
}

pub fn print_generated(quests: &[Quest]) {
    if quests.is_empty() {
        println!("No quests generated.");
        return;
    }

    println!("Generated {} quest(s):", quests.len());
    for quest in quests {
        print_one(quest);
    }
}

pub fn print_added(quest: &Quest) {
    println!("Quest added:");
    print_one(quest);
}

pub fn print_list(quests: &[Quest]) {
    if quests.is_empty() {
        println!("Quest board is empty. Try: cargo run -- generate 3");
        return;
    }

    println!("Quest board:\n");
    for quest in quests {
        print_one(quest);
    }
}

pub fn print_stats(quests: &[Quest]) {
    if quests.is_empty() {
        println!("No quests yet.");
        return;
    }

    let total = quests.len();
    let completed = quests
        .iter()
        .filter(|quest| matches!(quest.status, Status::Done))
        .count();
    let open = total - completed;
    let reward_total: u32 = quests.iter().map(|quest| quest.reward).sum();
    let reward_done: u32 = quests
        .iter()
        .filter(|quest| matches!(quest.status, Status::Done))
        .map(|quest| quest.reward)
        .sum();

    let average_difficulty = quests.iter().map(|quest| quest.difficulty as f32).sum::<f32>() / total as f32;

    println!("Quest stats");
    println!("-----------");
    println!("Total quests      : {}", total);
    println!("Open quests       : {}", open);
    println!("Completed quests  : {}", completed);
    println!("Potential rewards : {} gold", reward_total);
    println!("Collected rewards : {} gold", reward_done);
    println!("Avg difficulty    : {:.2}", average_difficulty);
}

fn print_one(quest: &Quest) {
    let marker = match quest.status {
        Status::Todo => "[ ]",
        Status::Done => "[x]",
    };

    println!(
        "{} #{} {} | diff {} | {} gold | {} | {} day(s)",
        marker,
        quest.id,
        quest.title,
        quest.difficulty,
        quest.reward,
        quest.biome,
        quest.deadline_days
    );
}
