use crate::config::{MISSPELLS_TABLE, RESPONSE_TABLE};
use std::collections::HashSet;
use std::sync::LazyLock;

pub struct Flags {
    flags: HashSet<u8>
}

impl Flags {
    pub fn has_flag(&self, index: u8) -> bool {
        self.flags.contains(&index)
    }
    pub fn has_response(&self) -> bool {
        self.has_flag(Self::RESPONSE.1)
    }
    pub fn has_misspell(&self) -> bool {
        self.has_flag(Self::MISSPELL.1)
    }

    pub const RESPONSE: (&str, u8) = ("response", 0);
    pub const MISSPELL: (&str, u8) = ("misspell", 1);
}

static FLAGS: &[&str] = &[
    Flags::RESPONSE.0, Flags::MISSPELL.0
];

static FLAG_LIST: LazyLock<Vec<Vec<&str>>> = LazyLock::new(|| {
    let mut array = vec![vec![]; FLAGS.len()];

    array[0] = RESPONSE_TABLE
        .into_iter()
        .flat_map(|(words, _)| words.split_ascii_whitespace())
        .collect();

    array[1] = MISSPELLS_TABLE.to_vec();
    array
});

pub fn get_message_flags(msg: &str) -> Flags {
    let mut flags: HashSet<u8> = HashSet::new();

    for flag in 0..FLAGS.len() {
        if FLAG_LIST[flag].iter().any(|flag| msg.contains(*flag)) {
            flags.insert(flag as u8);
        }
    }

    Flags { flags }
}
