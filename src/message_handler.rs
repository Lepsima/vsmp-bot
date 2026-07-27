use crate::config::{MISSPELLS_TABLE, RESPONSE_TABLE};
use crate::message_flagger::get_message_flags;
use serenity::all::Message;
use serenity::client::Context;

fn catch_error(res: serenity::Result<Message>) {
    if let Err(why) = res {
        println!("Error sending message: {why:?}");
    }
}

pub async fn handle_message(ctx: Context, msg: Message) {
    let text_lc = msg.content.to_lowercase();
    let flags = get_message_flags(&text_lc);

    if flags.has_response() {
        for response in RESPONSE_TABLE {
            if text_lc.contains(response.0) {
                let reply = response.1.to_string();
                catch_error(msg.channel_id.say(&ctx.http, reply).await);
                return;
            }
        }
    }

    if flags.has_misspell() {
        for response in MISSPELLS_TABLE {
            if text_lc.contains(response) {
                let reply = response.to_string() + " 🥺";
                catch_error(msg.channel_id.say(&ctx.http, reply).await);
                return;
            }
        }
    }
}