use crate::config::MISSPELLS_TABLE;
use crate::config::RESPONSE_TABLE;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static ID_TO_VOCAB: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let mut vocab: Vec<&str> = vec![""];

    RESPONSE_TABLE.iter().for_each(|response| {
        let mut words: Vec<&str> = response.0.split_ascii_whitespace().collect();
        vocab.append(&mut words);
    });

    MISSPELLS_TABLE.iter().for_each(|misspell| {
        vocab.push(misspell);
    });

    vocab
});

static VOCAB_TO_ID: LazyLock<HashMap<&'static str, usize>> = LazyLock::new(|| {
    ID_TO_VOCAB.iter()
        .enumerate()
        .skip(1)
        .map(|(i, w)| (*w, i))
        .collect()
});

pub struct QuickMessage {
    words: Vec<usize>,
    unique_words: HashSet<usize>
}

impl QuickMessage {
    pub fn has_word(&self, word: &str) -> bool {
        let id = VOCAB_TO_ID.get(word).unwrap_or(&0);
        self.words.contains(id)
    }

    pub fn has_words(&self, words: &Vec<&str>) -> bool {
        words.iter().all(|w| self.has_word(w))
    }

    pub fn has_any_words(&self, words: &Vec<&str>) -> bool {
        words.iter().any(|w| self.has_word(w))
    }


    pub fn has_id(&self, id: &usize) -> bool {
        self.words.contains(id)
    }

    pub fn has_ids(&self, ids: &Vec<usize>) -> bool {
        ids.iter().all(|w| self.has_id(w))
    }

    pub fn has_any_ids(&self, ids: &Vec<usize>) -> bool {
        ids.iter().any(|w| self.has_id(w))
    }
}

pub fn get_quick_message(msg: &str) -> QuickMessage {
    let words: Vec<usize> = tokenize(msg);
    let unique_words: HashSet<usize> = tokenize_unique(&words);

    QuickMessage {
        words,
        unique_words
    }
}

pub fn tokenize(text: &str) -> Vec<usize> {
    text.split_ascii_whitespace()
        .map(|token| *VOCAB_TO_ID.get(token).unwrap_or(&0))
        .collect()
}

pub fn tokenize_unique(tokens: &Vec<usize>) -> HashSet<usize> {
    HashSet::from_iter(tokens.iter().cloned())
}