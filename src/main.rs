mod config;
mod message_handler;
mod message_flagger;
mod cards;
mod blackjack;
mod poker;
mod data;

use std::{
    collections::HashSet,
    sync::Arc,
    time::Duration,
};
use std::collections::HashMap;
use crate::{
    config::Dialogue,
    message_handler::handle_message
};

use serenity::{
    async_trait,
    model::{
        channel::Message,
        application::{interaction::Interaction, component::ButtonStyle}
    },
    prelude::*,
    builder::{CreateActionRow, CreateButton, EditMessage},
};

use bollard::{Docker, query_parameters::EventsOptionsBuilder};
use craftping::tokio::ping;
use futures_util::StreamExt;
use poise::CreateReply;
use poise::serenity_prelude::{self as serenity, ChannelId, CreateEmbed};
use serenity::all::{CreateActionRow, UserId};
use tokio::{net::TcpStream, sync::Mutex};
use tracing::{error, info};
use crate::blackjack::BlackJack;
use crate::data::ScoreDb;

const MC_HOST: &str = "host.docker.internal";
const MC_PORT: u16 = 25565;
const MC_CONTAINER_NAME: &str = "purpur";
const STATUS_CHANNEL_ID: u64 = 1482111537127620608;
const ACTIVITY_CHANNEL_ID: u64 = 1482122160930689216;
const GAMBLING_CHANNEL_ID: u64 = 1526711423412211802;

struct Color;

impl Color {
    pub const GREEN: u32 = 0x57F287u32;
    pub const ORANGE: u32 = 0xFFA500u32;
    pub const RED: u32 = 0xED4245u32;
    pub const BLUE: u32 = 0x5865F2u32;
}

struct Handler {
    data: Arc<Data>,
}

struct Data {
    mc_host: String,
    mc_port: u16,
    last_player_set: Mutex<HashSet<String>>,
    last_activity_message: Mutex<Option<serenity::MessageId>>,
    black_jack: Mutex<HashMap<u64, BlackJack>>,
    notify_role: Mutex<bool>,
    scores: Mutex<ScoreDb>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Arc<Data>, Error>;

struct McStatus {
    online: usize,
    max: usize,
    players: Vec<String>
}

async fn get_mc_status(host: &str, port: u16) -> Option<McStatus> {
    let mut stream = TcpStream::connect((host, port)).await.ok()?;
    let pong = ping(&mut stream, host, port).await.ok()?;

    let players = pong
        .sample
        .unwrap_or_default()
        .into_iter()
        .map(|p| p.name)
        .collect();

    Some(McStatus {
        online: pong.online_players,
        max: pong.max_players,
        players
    })
}

fn embed_players_online(status: &McStatus) -> CreateEmbed {
    let description = if status.online == 0 || status.players.is_empty() {
        Dialogue::SERVER_EMPTY.to_string()
    } else {
        let names = status
            .players
            .iter()
            .map(|n| format!("• {n}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "**Players ({}/{}):**\n{}",
            status.online, status.max, names
        )
    };

    let color = if status.online == 0 {
        Color::ORANGE
    } else {
        Color::GREEN
    };

    create_embed(true, &description, color)
}

fn embed_server_online(ready: bool) -> CreateEmbed {
    if ready {
        create_embed(true, Dialogue::SERVER_READY, Color::GREEN)
    } else {
        create_embed(true, Dialogue::SERVER_NOT_READY, Color::ORANGE)
    }
}

fn embed_players_offline() -> CreateEmbed {
    create_embed(false, Dialogue::UNABLE_TO_CONNECT, Color::RED)
}

fn embed_server_offline() -> CreateEmbed {
    create_embed(false, Dialogue::SERVER_DOWN, Color::RED)
}

fn create_embed(is_online: bool, desc: &str, color: u32) -> CreateEmbed {
    let embed = CreateEmbed::new()
        .description(desc)
        .color(color);

    if is_online {
        embed.title(Dialogue::SERVER_ONLINE)
    } else {
        embed.title(Dialogue::SERVER_OFFLINE)
    }
}

fn is_blackjack_channel(ctx: &Context<'_>) -> bool {
    ctx.channel_id().get() != GAMBLING_CHANNEL_ID
}

#[poise::command(slash_command, rename = "play-blackjack")]
async fn play_blackjack(ctx: Context<'_>, #[description = "Bet amount"] bet: usize) -> Result<(), Error> {
    ctx.defer().await?;

    if is_blackjack_channel(&ctx) {
        ctx.say("No blackjack here").await?;
        return Ok(());
    }

    let mut scores = ctx.data().scores.lock().await;
    let user_id = ctx.author().id;

    let score = scores.get(user_id);
    if (bet as i128) > score {
        ctx.say(format!("Insufficient dolla, current amount: {}", score)).await?;
        return Ok(());
    }

    let bet = usize::min(score as usize, bet);
    let mut blackjacks = ctx.data().black_jack.lock().await;
    if !blackjacks.contains_key(&user_id.get()) {
        blackjacks.insert(user_id.get(), BlackJack::new(&user_id));
    }

    let blackjack = blackjacks.get_mut(&user_id.get()).unwrap();
    if !blackjack.is_playing {
        let embed = blackjack.play(bet, &mut scores);
        ctx.send(CreateReply::default().embed(embed)).await?;
    } else {
        ctx.say("Already playing...").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
async fn blackjack(ctx: Context<'_>, #[description = "hit, stand, double down"] action: String) -> Result<(), Error> {
    ctx.defer().await?;

    if is_blackjack_channel(&ctx) {
        ctx.say("No blackjack here").await?;
        return Ok(());
    }

    let user_id = ctx.author().id;
    let mut blackjacks = ctx.data().black_jack.lock().await;

    if !blackjacks.contains_key(&user_id.get()) {
        ctx.say("use 'play-blackjack' to start a game").await?;
        return Ok(());
    }

    let blackjack = blackjacks.get_mut(&user_id.get()).unwrap();
    if !blackjack.is_playing {
        ctx.say("use 'play-blackjack' to start a game").await?;
        return Ok(());
    }

    let mut scores = ctx.data().scores.lock().await;
    let embed = blackjack.turn(&action, &mut scores);

    let components = vec![
        CreateActionRow::Buttons(vec![
            CreateButton::new("hit").label("Hit").style(ButtonStyle::Primary),
            CreateButton::new("std").label("Stand").style(ButtonStyle::Success),
            CreateButton::new("dbl").label("Double").style(ButtonStyle::Danger),
        ])
    ];

    ctx.send(CreateReply::default().embed(embed).components(components)).await?;

    if !blackjack.is_playing {
        blackjack.clear();
    }

    Ok(())
}

#[poise::command(slash_command, rename = "one-dolla")]
async fn one_dolla(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let mut scores = ctx.data().scores.lock().await;
    scores.set(ctx.author().id, 1);

    let embed = CreateEmbed::new()
        .title("Mrbeast free money")
        .description("you have one(1) dolla")
        .color(Color::GREEN);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let scores = ctx.data().scores.lock().await;
    let mut names: String = "".to_string();
    let mut number = 0;

    let mut all: Vec<(UserId, i128)> = scores.all().collect();
    all.sort_by(|a, b| b.1.cmp(&a.1));

    for entry in all {
        number += 1;
        let user = entry.0.to_user(ctx.http()).await;
        let name = user?.name;
        names += &format!("-{}: {} has {} dolla\n", number, name, entry.1);
    }

    let embed = CreateEmbed::new()
        .title("dolla Leaderboard")
        .description(names)
        .color(Color::BLUE);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}


#[poise::command(slash_command)]
async fn players(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let opt_status = get_mc_status(&ctx.data().mc_host, ctx.data().mc_port).await;
    let embed: CreateEmbed = match opt_status {
        Some(status) => embed_players_online(&status),
        None => embed_players_offline(),
    };

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command, rename = "notify-empty")]
async fn notify_empty(ctx: Context<'_>) -> Result<(), Error> {
    let mut notify = ctx.data().notify_role.lock().await;

    if *notify {
        *notify = false;
        return Ok(());
    }

    match get_mc_status(&ctx.data().mc_host, ctx.data().mc_port).await {
        None => {
            ctx.say(Dialogue::UNABLE_TO_CONNECT).await?;
            return Ok(());
        }
        Some(status) if status.online == 0 => {
            ctx.say(Dialogue::ALREADY_EMPTY).await?;
            return Ok(());
        }
        _ => {}
    }

    *notify = true;
    ctx.say(Dialogue::WILL_PING).await?;
    Ok(())
}

async fn task_check_empty(ctx: serenity::Context, data: Arc<Data>) {
    loop {
        tokio::time::sleep(Duration::from_secs(5 * 60)).await;

        if !*data.notify_role.lock().await {
            continue;
        }

        match get_mc_status(&data.mc_host, data.mc_port).await {
            Some(status) if status.online == 0 => {
                let channel = ChannelId::new(STATUS_CHANNEL_ID);
                if let Err(e) = channel.say(&ctx, Dialogue::WHEN_EMPTY).await {
                    error!("Failed to send notify-empty message: {e}");
                }

                *data.notify_role.lock().await = false;
            }
            None => {
                error!("notify-empty: could not reach MC server");
            }
            _ => {}
        }
    }
}

async fn task_check_players(ctx: serenity::Context, data: Arc<Data>) {
    loop {
        tokio::time::sleep(Duration::from_secs(15 * 60)).await;

        let status = match get_mc_status(&data.mc_host, data.mc_port).await {
            Some(s) => s,
            None => continue,
        };

        ctx.set_presence(
            Some(serenity::ActivityData::playing(format!(
                "{} players online",
                status.online
            ))),
            serenity::OnlineStatus::Online,
        );

        if status.online < 3 || status.players.is_empty() {
            *data.last_player_set.lock().await = HashSet::new();
            continue;
        }

        let current: HashSet<String> = status.players.iter().cloned().collect();

        {
            let mut last = data.last_player_set.lock().await;
            if *last == current {
                continue;
            }
            *last = current.clone();
        }

        let channel = ChannelId::new(ACTIVITY_CHANNEL_ID);

        // Delete previous activity message if any
        {
            let mut last_msg = data.last_activity_message.lock().await;
            if let Some(msg_id) = last_msg.take() {
                let _ = channel.delete_message(&ctx, msg_id).await;
            }
        }

        let names = current
            .iter()
            .map(|n| format!("• {n}"))
            .collect::<Vec<_>>()
            .join("\n");

        let embed = CreateEmbed::new()
            .title(Dialogue::ACTIVE_SESSION)
            .description(format!(
                "There are **{}** players online:\n{}",
                status.online, names
            ))
            .color(Color::BLUE);

        match channel
            .send_message(&ctx, serenity::CreateMessage::new().embed(embed))
            .await
        {
            Ok(msg) => {
                *data.last_activity_message.lock().await = Some(msg.id);
            }
            Err(e) => error!("Failed to send activity embed: {e}"),
        }
    }
}

pub fn catch_msg(res: serenity::Result<Message>) {
    if let Err(why) = res {
        println!("Error sending message: {why:?}");
    }
}

/// Listens to Docker container events for start/die on the MC container.
async fn task_docker_events(ctx: serenity::Context, data: Arc<Data>) {
    let docker = match Docker::connect_with_socket_defaults() {
        Ok(d) => d,
        Err(e) => {
            error!("Could not connect to Docker socket: {e}");
            return;
        }
    };

    let mut filters = HashMap::new();
    filters.insert("container", vec![MC_CONTAINER_NAME]);
    filters.insert("type", vec!["container"]);

    // bollard 0.21 uses EventsOptionsBuilder from query_parameters
    let options = EventsOptionsBuilder::default()
        .filters(&filters)
        .build();

    let mut stream = docker.events(Some(options));

    while let Some(event) = stream.next().await {
        match event {
            Ok(ev) => {
                let action = ev.action.as_deref().unwrap_or("");
                match action {
                    "start" => on_mc_start(ctx.clone(), data.clone()).await,
                    "die" => on_mc_stop(ctx.clone()).await,
                    _ => {}
                }
            }
            Err(e) => error!("Docker event error: {e}"),
        }
    }
}

async fn on_mc_start(ctx: serenity::Context, data: Arc<Data>) {
    info!("MC container started, waiting 80s...");
    tokio::time::sleep(Duration::from_secs(80)).await;

    let channel = ChannelId::new(STATUS_CHANNEL_ID);
    let ready = get_mc_status(&data.mc_host, data.mc_port).await.is_some();
    let embed = embed_server_online(ready);

    if let Err(e) = channel
        .send_message(&ctx, serenity::CreateMessage::new().embed(embed))
        .await
    {
        error!("Failed to send server-online message: {e}");
    }

    ctx.set_presence(
        Some(serenity::ActivityData::playing(Dialogue::SERVER_ONLINE)),
        serenity::OnlineStatus::Online,
    );
}

async fn on_mc_stop(ctx: serenity::Context) {
    info!("MC container stopped.");
    let channel = ChannelId::new(STATUS_CHANNEL_ID);
    let embed = embed_server_offline();

    if let Err(e) = channel
        .send_message(&ctx, serenity::CreateMessage::new().embed(embed))
        .await
    {
        error!("Failed to send server-offline message: {e}");
    }

    ctx.set_presence(
        Some(serenity::ActivityData::playing(Dialogue::SERVER_OFFLINE)),
        serenity::OnlineStatus::DoNotDisturb,
    );
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, msg: Message) {
        if !msg.author.bot {
            handle_message(ctx, msg).await;
        }
    }

    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready) {
        info!("Logged in as {}", ready.user.name);

        ctx.set_presence(
            Some(serenity::ActivityData::playing(Dialogue::SERVER_STARTING)),
            serenity::OnlineStatus::Idle,
        );

        tokio::spawn(task_check_empty(ctx.clone(), self.data.clone()));
        tokio::spawn(task_check_players(ctx.clone(), self.data.clone()));
        tokio::spawn(task_docker_events(ctx.clone(), self.data.clone()));
    }

    async fn interaction_create(&self, ctx: serenity::Context, interaction: Interaction) {
        if let Interaction::Component(component) = interaction {
            let id = component.data.custom_id.as_str();
            if !matches!(id, "hit" | "std" | "dbl") {
                return;
            }

            component.defer(&ctx).await.unwrap();

            let user_id = component.user.id;
            let mut blackjacks = self.data.black_jack.lock().await;

            if !blackjacks.contains_key(&user_id.get()) {
                let reply = "use 'play-blackjack' to start a game";
                catch_msg(component.channel_id.say(&ctx.http, reply).await);
                return;
            }

            let blackjack = blackjacks.get_mut(&user_id.get()).unwrap();
            if !blackjack.is_playing {
                let reply = "use 'play-blackjack' to start a game";
                catch_msg(component.channel_id.say(&ctx.http, reply).await);
                return;
            }

            let action;
            match id {
                "hit" => { action = "hit" }
                "std" => { action = "stand" }
                "dbl" => { action = "double down" }
                _ => {}
            }

            let mut scores = self.data.scores.lock().await;
            let embed = blackjack.turn(&action, &mut scores);

            let components = vec![
                CreateActionRow::Buttons(vec![
                    CreateButton::new("hit").label("Hit").style(ButtonStyle::Primary),
                    CreateButton::new("std").label("Stand").style(ButtonStyle::Success),
                    CreateButton::new("dbl").label("Double").style(ButtonStyle::Danger),
                ])
            ];

            component.message.clone().edit(&ctx, EditMessage::new()
                .embed(embed)
                .components(vec![])
            ).await.unwrap();

            if !blackjack.is_playing {
                blackjack.clear();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    let data = Arc::new(Data {
        mc_host: MC_HOST.to_string(),
        mc_port: MC_PORT,
        last_player_set: Mutex::new(HashSet::new()),
        last_activity_message: Mutex::new(None),
        notify_role: Mutex::new(false),
        black_jack: Mutex::new(HashMap::new()),
        scores: Mutex::new(ScoreDb::open("./data/scores")),
    });

    let data_clone = data.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![one_dolla(), leaderboard(), play_blackjack(), blackjack(), players(), notify_empty()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data_clone)
            })
        })
        .build();

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .event_handler(Handler { data: data.clone() })
        .framework(framework)
        .await
        .expect("Failed to create client");

    if let Err(e) = client.start().await {
        error!("Client error: {e}");
    }
}