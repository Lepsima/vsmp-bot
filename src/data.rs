use std::{collections::HashMap, path::PathBuf};
use serenity::model::id::UserId;

pub struct ScoreDb {
    path: PathBuf,
    data: HashMap<u64, i128>,
}

impl ScoreDb {
    pub const DEFAULT: i128 = 10_000;

    pub fn open(path: &str) -> Self {
        let path = PathBuf::from(path);
        let data = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Self { path, data }
    }

    pub fn get(&self, user_id: UserId) -> i128 {
        self.data
            .get(&user_id.get())
            .copied()
            .unwrap_or(ScoreDb::DEFAULT)
    }

    pub fn set(&mut self, user_id: UserId, value: i128) {
        self.data.insert(user_id.get(), value);
        let json = serde_json::to_string(&self.data).unwrap();
        std::fs::write(&self.path, json).expect("Failed to write scores");
    }

    pub fn all(&self) -> impl Iterator<Item = (UserId, i128)> + '_ {
        self.data
            .iter()
            .map(|(&uid, &val)| (UserId::new(uid), val))
    }
}