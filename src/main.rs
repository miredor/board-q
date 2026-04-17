mod generator;
mod models;
mod storage;
mod ui;

use std::env;
use std::error::Error;
use std::path::PathBuf;

use generator::{generate_many, make_custom_quest};
use models::{Quest, Status};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let db_path = PathBuf::from("quests.db");
    let mut quests = storage::load(&db_path)?;

    match args.get(1).map(String::as_str) {
        Some("generate") => {
            let count = args
                .get(2)
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(3);

            let start_id = next_id(&quests);
            let mut fresh = generate_many(count);

            for (index, quest) in fresh.iter_mut().enumerate() {
                quest.id = start_id + index as u32;
            }

            let preview = fresh.clone();
            quests.extend(fresh);
            storage::save(&db_path, &quests)?;
            ui::print_generated(&preview);
        }
        Some("add") => {
            if args.len() < 3 {
                ui::print_help();
                return Ok(());
            }

            let title = args[2..].join(" ");
            let quest = make_custom_quest(next_id(&quests), title);
            quests.push(quest.clone());
            storage::save(&db_path, &quests)?;
            ui::print_added(&quest);
        }
        Some("list") => {
            ui::print_list(&quests);
        }
        Some("done") => {
            let Some(id_raw) = args.get(2) else {
                ui::print_help();
                return Ok(());
            };

            let id = id_raw.parse::<u32>()?;
            let mut updated = false;

            for quest in &mut quests {
                if quest.id == id {
                    quest.status = Status::Done;
                    updated = true;
                    break;
                }
            }

            if updated {
                storage::save(&db_path, &quests)?;
                println!("Quest #{id} completed.");
            } else {
                println!("Quest #{id} not found.");
            }
        }
        Some("stats") => {
            ui::print_stats(&quests);
        }
        Some("help") | None => {
            ui::print_help();
        }
        Some(other) => {
            println!("Unknown command: {other}\n");
            ui::print_help();
        }
    }

    Ok(())
}

fn next_id(quests: &[Quest]) -> u32 {
    quests.iter().map(|quest| quest.id).max().unwrap_or(0) + 1
}
