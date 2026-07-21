static RESPONSE_TABLE: &[(&str, &str)] = &[
    ("crazy", "Crazy? I was crazy once.\nThey put me in a room."),
    ("rubber room", "A rubber room with rats\nThey put me in a rubber room with rubber rats."),
    ("rubber rats", "I hate rubber rats.\nThey make me crazy."),
    ("jewish burger", "Jewish burger"),
    ("indelible", "INDELIBLE tomorrow"),
    ("spiffballs", "<@967923910253363201>, Skibidi Spiffballs for 10$."),
    ("mayo", "Mayo, mmmmmmm~")
];

pub fn get_response(msg: &String) -> Option<String> {
    let msg = msg.to_lowercase();

    for response in RESPONSE_TABLE {
        if msg.contains(response.0) {
            return Some(response.1.to_string());
        }
    }

    None
}
