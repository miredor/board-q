#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Todo,
    Done,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Todo => "todo",
            Status::Done => "done",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "todo" => Some(Status::Todo),
            "done" => Some(Status::Done),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Quest {
    pub id: u32,
    pub title: String,
    pub difficulty: u8,
    pub reward: u32,
    pub biome: String,
    pub deadline_days: u8,
    pub status: Status,
}

impl Quest {
    pub fn to_record(&self) -> String {
        let safe_title = self.title.replace('\t', " ").replace('\n', " ");
        let safe_biome = self.biome.replace('\t', " ").replace('\n', " ");

        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.id,
            safe_title,
            self.difficulty,
            self.reward,
            safe_biome,
            self.deadline_days,
            self.status.as_str()
        )
    }

    pub fn from_record(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 7 {
            return None;
        }

        Some(Self {
            id: parts[0].parse().ok()?,
            title: parts[1].to_string(),
            difficulty: parts[2].parse().ok()?,
            reward: parts[3].parse().ok()?,
            biome: parts[4].to_string(),
            deadline_days: parts[5].parse().ok()?,
            status: Status::from_str(parts[6])?,
        })
    }
}
