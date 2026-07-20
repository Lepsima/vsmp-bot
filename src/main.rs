use std::{
    collections::HashSet,
    sync::Arc,
    time::Duration,
};

use bollard::{Docker, query_parameters::EventsOptionsBuilder};
use craftping::tokio::ping;
use futures_util::StreamExt;
use poise::serenity_prelude::{self as serenity, ChannelId, CreateEmbed, Mentionable, RoleId};
use tokio::{net::TcpStream, sync::Mutex};
use tracing::{error, info};

// ========== CONFIG ==========

const MC_HOST: &str = "host.docker.internal";
const MC_PORT: u16 = 25565;
const MC_CONTAINER_NAME: &str = "purpur";
const STATUS_CHANNEL_ID: u64 = 1482111537127620608;
const ACTIVITY_CHANNEL_ID: u64 = 1482122160930689216;

// ========== SHARED STATE ==========

struct Data {
    mc_host: String,
    mc_port: u16,
    last_player_set: Mutex<HashSet<String>>,
    last_activity_message: Mutex<Option<serenity::MessageId>>,
    notify_role_id: Mutex<Option<RoleId>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Arc<Data>, Error>;

// ========== MC STATUS ==========

struct McStatus {
    online: usize,
    max: usize,
    players: Vec<String>,
    latency_ms: u64,
}

/// Opens a TcpStream and pings the MC server. Returns None if unreachable.
async fn get_mc_status(host: &str, port: u16) -> Option<McStatus> {
    let start = std::time::Instant::now();
    // craftping requires us to open the stream ourselves
    let mut stream = TcpStream::connect((host, port)).await.ok()?;
    let pong = ping(&mut stream, host, port).await.ok()?;
    let latency_ms = start.elapsed().as_millis() as u64;

    let players = pong
        .sample
        .unwrap_or_default()
        .into_iter()
        .map(|p| p.name)
        .collect();

    Some(McStatus {
        online: pong.online_players,
        max: pong.max_players,
        players,
        latency_ms,
    })
}

// ========== EMBEDS ==========

fn embed_players_online(status: &McStatus) -> CreateEmbed {
    let description = if status.online == 0 || status.players.is_empty() {
        "Nobody is online".to_string()
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
        0xFFA500u32 // orange
    } else {
        0x57F287 // green
    };

    CreateEmbed::new()
        .title("Server is online")
        .description(description)
        .footer(serenity::CreateEmbedFooter::new(format!(
            "Ping: {}ms",
            status.latency_ms
        )))
        .color(color)
}

fn embed_players_offline() -> CreateEmbed {
    CreateEmbed::new()
        .title("Server is offline")
        .description("Unable to connect to the Minecraft server.")
        .color(0xED4245u32)
}

fn embed_server_online(ready: bool) -> CreateEmbed {
    if ready {
        CreateEmbed::new()
            .title("Server online")
            .description("The server is ready.")
            .color(0x57F287u32)
    } else {
        CreateEmbed::new()
            .title("Server online")
            .description(
                "The server is opening slower than expected, have patience when trying to join.",
            )
            .color(0xFFA500u32)
    }
}

fn embed_server_offline() -> CreateEmbed {
    CreateEmbed::new()
        .title("Server offline")
        .description("The Minecraft server has shut down.")
        .color(0xED4245u32)
}

// ========== COMMANDS ==========

/// Displays the active players in the server
#[poise::command(slash_command)]
async fn players(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let embed = match get_mc_status(&ctx.data().mc_host, ctx.data().mc_port).await {
        Some(status) => embed_players_online(&status),
        None => embed_players_offline(),
    };

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Ping a role when the server is empty (toggle)
#[poise::command(slash_command, rename = "notify-empty")]
async fn notify_empty(
    ctx: Context<'_>,
    #[description = "Role to ping when the server is empty"] role: serenity::Role,
) -> Result<(), Error> {
    let mut notify = ctx.data().notify_role_id.lock().await;

    // Toggle off if same role is set
    if *notify == Some(role.id) {
        *notify = None;
        ctx.say("Notification cancelled.").await?;
        return Ok(());
    }

    // Check server is reachable and not already empty
    match get_mc_status(&ctx.data().mc_host, ctx.data().mc_port).await {
        None => {
            ctx.say("Could not connect to the server.").await?;
            return Ok(());
        }
        Some(status) if status.online == 0 => {
            ctx.say("The server is already empty.").await?;
            return Ok(());
        }
        _ => {}
    }

    *notify = Some(role.id);
    // role.id implements Mentionable, imported above
    ctx.say(format!(
        "I'll ping {} when the server is empty.",
        role.id.mention()
    ))
        .await?;
    Ok(())
}

// ========== BACKGROUND TASKS ==========

/// Every 5 minutes: if a notify role is set and server is empty, ping it.
async fn task_check_empty(ctx: serenity::Context, data: Arc<Data>) {
    loop {
        tokio::time::sleep(Duration::from_secs(5 * 60)).await;

        let role_id = *data.notify_role_id.lock().await;
        let Some(role_id) = role_id else { continue };

        match get_mc_status(&data.mc_host, data.mc_port).await {
            Some(status) if status.online == 0 => {
                let channel = ChannelId::new(STATUS_CHANNEL_ID);
                if let Err(e) = channel
                    .say(&ctx, format!("{} The server is now empty.", role_id.mention()))
                    .await
                {
                    error!("Failed to send notify-empty message: {e}");
                }
                *data.notify_role_id.lock().await = None;
            }
            None => {
                error!("notify-empty: could not reach MC server");
            }
            _ => {} // still players online, keep waiting
        }
    }
}

/// Every 15 minutes: if ≥3 players online and player list changed, post activity embed.
async fn task_check_players(ctx: serenity::Context, data: Arc<Data>) {
    loop {
        tokio::time::sleep(Duration::from_secs(15 * 60)).await;

        let status = match get_mc_status(&data.mc_host, data.mc_port).await {
            Some(s) => s,
            None => continue,
        };

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
            .title("Active session")
            .description(format!(
                "There are **{}** players online:\n{}",
                status.online, names
            ))
            .color(0x5865F2u32);

        match channel
            .send_message(&ctx, serenity::CreateMessage::new().embed(embed))
            .await
        {
            Ok(msg) => {
                *data.last_activity_message.lock().await = Some(msg.id);
            }
            Err(e) => error!("Failed to send activity embed: {e}"),
        }

        ctx.set_presence(
            Some(serenity::ActivityData::playing(format!(
                "{} players online",
                status.online
            ))),
            serenity::OnlineStatus::Online,
        );
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

    let mut filters = std::collections::HashMap::new();
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
        Some(serenity::ActivityData::playing("Server is online")),
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
        Some(serenity::ActivityData::playing("Server offline")),
        serenity::OnlineStatus::DoNotDisturb,
    );
}

// ========== EVENT HANDLER ==========

struct Handler {
    data: Arc<Data>,
}

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready) {
        info!("Logged in as {}", ready.user.name);

        ctx.set_presence(
            Some(serenity::ActivityData::playing("Server starting...")),
            serenity::OnlineStatus::Idle,
        );

        tokio::spawn(task_check_empty(ctx.clone(), self.data.clone()));
        tokio::spawn(task_check_players(ctx.clone(), self.data.clone()));
        tokio::spawn(task_docker_events(ctx.clone(), self.data.clone()));
    }
}

// ========== MAIN ==========

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
        notify_role_id: Mutex::new(None),
    });

    let data_clone = data.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![players(), notify_empty()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data_clone)
            })
        })
        .build();

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .event_handler(Handler { data: data.clone() })
        .framework(framework)
        .await
        .expect("Failed to create client");

    if let Err(e) = client.start().await {
        error!("Client error: {e}");
    }
}