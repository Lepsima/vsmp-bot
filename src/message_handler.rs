use std::sync::LazyLock;
use crate::config::{MISSPELLS_TABLE, RESPONSE_TABLE};
use crate::message_parser::{get_quick_message, tokenize, QuickMessage};

static QUICK_RESPONSE_TABLE: LazyLock<Vec<(Vec<usize>, &str)>> = LazyLock::new(|| {
    let mut qrt: Vec<(Vec<usize>, &str)> = Vec::new();

    RESPONSE_TABLE.into_iter().for_each(|response| {
        qrt.push((tokenize(response.0), response.1));
    });

    qrt
});

pub fn get_response_1(msg: &String) -> Vec<String> {
    let msg = msg.to_lowercase();
    let mut vec = vec![];

    for response in RESPONSE_TABLE {
        if msg.contains(response.0) {
            vec.push(response.1.to_string());
        }
    }

    vec
}

pub fn get_response_2(msg: &String) -> Vec<String> {
    let quick_msg = get_quick_message(msg);
    get_response_3(&quick_msg)
}

pub fn get_response_3(msg: &QuickMessage) -> Vec<String> {
    let mut vec = vec![];


    for response in QUICK_RESPONSE_TABLE.iter() {
        if msg.has_ids(&response.0) {
            vec.push(response.1.to_string());
        }
    };

    vec
}