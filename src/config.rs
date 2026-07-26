pub struct Dialogue;

impl Dialogue {
    pub const SERVER_ONLINE: &str = "Server is online";
    pub const SERVER_OFFLINE: &str = "Server is offline";

    pub const SERVER_READY: &str = "The server is ready.";
    pub const SERVER_NOT_READY: &str = "The server is opening slower than expected, have patience when trying to join.";
    pub const SERVER_DOWN: &str = "The Minecraft server has shut down.";
    pub const SERVER_EMPTY: &str = "Nobody is online";

    pub const UNABLE_TO_CONNECT: &str = "Unable to connect to the Minecraft server.";
    pub const ALREADY_EMPTY: &str = "The server is already empty.";
    pub const WILL_PING: &str = "I'll ping when the server is empty.";
    pub const WHEN_EMPTY: &str = "<@&1527816106738192546> The server is now empty.";

    pub const SERVER_STARTING: &str = "Server starting...";
    pub const ACTIVE_SESSION: &str = "Active session";
}

pub static RESPONSE_TABLE: &[(&str, &str)] = &[
    ("crazy", "Crazy? I was crazy once.\nThey put me in a room."),
    ("rubber room", "A rubber room with rats\nThey put me in a rubber room with rubber rats."),
    ("rubber rats", "I hate rubber rats.\nThey make me crazy."),
    ("jewish burger", "Jewish burger"),
    ("indelible", "INDELIBLE tomorrow"),
    ("spiffballs", "<@967923910253363201>, Skibidi Spiffballs for 10$."),
    ("mayo", "Mayo, mmmmmmm~")
];

pub static MISSPELLS_TABLE: &[&str] = &[
    "lamo", "ahr", "forgor"
];