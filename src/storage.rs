use std::fs;
use std::io;
use std::path::Path;

use crate::models::Quest;

pub fn load(path: &Path) -> io::Result<Vec<Quest>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path)?;
    let quests = content
        .lines()
        .filter_map(Quest::from_record)
        .collect::<Vec<_>>();

    Ok(quests)
}

pub fn save(path: &Path, quests: &[Quest]) -> io::Result<()> {
    let content = quests
        .iter()
        .map(Quest::to_record)
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(path, content)
}
