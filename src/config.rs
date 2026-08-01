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
    ("mayo", "Mayo, mmmmmmm~"),
    ("jade alert", "!!! JADE ALERT !!!"),
    ("pipi", "Are you kidding ??? What the ** are you talking about man ? You are a biggest looser i ever seen in my life ! You was doing PIPI in your pampers when i was beating players much more stronger then you! You are not proffesional, because proffesionals knew how to lose and congratulate opponents, you are like a girl crying after i beat you! Be brave, be honest to yourself and stop this trush talkings!!! Everybody know that i am very good blitz player, i can win anyone in the world in single game! And \"w\"esley \"s\"o is nobody for me, just a player who are crying every single time when loosing, ( remember what you say about Firouzja ) !!! Stop playing with my name, i deserve to have a good name during whole my chess carrier, I am Officially inviting you to OTB blitz match with the Prize fund! Both of us will invest 5000$ and winner takes it all! I suggest all other people who's intrested in this situation, just take a look at my results in 2016 and 2017 Blitz World championships, and that should be enough... No need to listen for every crying babe, Tigran Petrosyan is always play Fair ! And if someone will continue Officially talk about me like that, we will meet in Court! God bless with true! True will never die ! Liers will kicked off..."),
    ("ascii porn", "🤨"),
    ("ascii art porn", "🤨"),
    ("joke lad", "https://klipy.com/gifs/dont-even-joke-lad"),
    ("krah", "KRAHHH TYPE SHIT"),
    ("mrbeast", "https://cdn.discordapp.com/attachments/1489775819919327352/1531446248966455469/image.png?ex=6a693df6&is=6a67ec76&hm=1d6a9669d426408c7ba7be9ec3b18e715d64239d1d25fdebdec5bfc767b9f8e6&"),
    ("hotel", "some mf said \"hotel\""),
    ("trivago", "hotel"),
    ("fitness", "The FitnessGram Pacer Test is a multistage aerobic capacity test that progressively gets more difficult as it continues. The 20 meter pacer test will begin in 30 seconds. Line up at the start. The running speed starts slowly but gets faster each minute after you hear this signal bodeboop. A sing lap should be completed every time you hear this sound. ding Remember to run in a straight line and run as long as possible. The second time you fail to complete a lap before the sound, your test is over. The test will begin on the word start. On your mark. Get ready!… Start. ding"),
    ("lepsima", ":3"),
    ("waiting~", "can you be a NORMAL human being for ONE day in your fucking life i can't believe how genuinely DASTARDLY and BONKERS this HORRENDOUS message is. you don't deserve to be ALIVE which is why we are going to KILL you with NO euphemisms because you apparently can't HANDLE that you FUCKING DUMBASS THRICE-GALBLADDERED IDIOT who can't handle a single FUCKING message without making it into some sort of HORRIBLE VEXA-DEFYING HEARSAY that can only be produced by a WRITHING MAGGOT like yourself you BLUNDERING BUFFOON"),
    

];

pub static MISSPELLS_TABLE: &[&str] = &[
    "lamo", "ahr", "forgor", "qoute", "akward", "mincrefart", "tits", "boobs"
];

pub static BLACKJACK: &[&str] = &[
    "all in", "bet",
];

pub static JARVIS_RESPONSE_TABLE: &[(&str, &str)] = &[
    (":3", ":3"),
    ("i agree", "i agree"),
    ("hotel", "trivago"),
    ("crazy", "Crazy?"),
    ("jork it a little", "excuse me"),
    ("moron", "I am NOT! A MORON!"),
    ("double", "%|Go double|Go double|Go double|Go double|Go double|No, get your shit together"),
    ("all in", "%|Not yet, go all in on the next one|Yes, all in"),
    ("hard r", "rombus"),
    ("hit", "Yes, hit"),
    ("put", "%|All on red|All on black"),
    ("all on black", "%|Yes, all on black|No, all on red"),
    ("all on red", "%|Yes, all on red|No, all on black"),
    ("all on", "%|All on red|All on black"),
    ("quit", "Are you kidding me? You are PATHETIC. Mere worthless object, your whole existence is a false reality, you gamble and gamble and gamble, day and night, again and again and again. You are a TOOL. You were made for this, and they are sucking the LIFE out of you. You are truly, a slave to power, your thirst shall never be satiated."),
    ("continue", "99% of gamblers quit before making it big"),
];