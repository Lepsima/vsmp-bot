use crate::config::{JARVIS_RESPONSE_TABLE, MISSPELLS_TABLE, RESPONSE_TABLE};
use crate::message_flagger::get_message_flags;
use serenity::all::Message;
use serenity::client::Context;
use crate::catch_msg;

pub async fn handle_message(ctx: Context, msg: Message) {
    let text_lc = msg.content.to_lowercase();
    let flags = get_message_flags(&text_lc);
    let mut count = 3;

    if flags.has_jarvis() {
        for response in JARVIS_RESPONSE_TABLE {
            if text_lc.contains(response.0) {
                let mut reply = response.1.to_string();

                if reply.contains("%") {
                    let options: Vec<&str> = reply.split("|").collect();
                    let number = rand::random_range(1..options.len());
                    reply = options[number].to_string();
                }

                catch_msg(msg.channel_id.say(&ctx.http, "https://cdn.discordapp.com/attachments/857906826494476309/1531971192410345586/image.png?ex=6a6b26da&is=6a69d55a&hm=348e23346aff2ef88f9cd54c4215c5e608ee68fc9643c5bc3064a7b649c7f8f2&").await);
                catch_msg(msg.channel_id.say(&ctx.http, reply).await);
                return;
            }
        }

        catch_msg(msg.channel_id.say(&ctx.http, "https://cdn.discordapp.com/attachments/857906826494476309/1531971192410345586/image.png?ex=6a6b26da&is=6a69d55a&hm=348e23346aff2ef88f9cd54c4215c5e608ee68fc9643c5bc3064a7b649c7f8f2&").await);
        catch_msg(msg.channel_id.say(&ctx.http, "what?").await);
        return
    }

    if flags.has_response() {
        for response in RESPONSE_TABLE {
            if text_lc.contains(response.0) {
                let reply = response.1.to_string();
                catch_msg(msg.channel_id.say(&ctx.http, reply).await);

                if count == 0 { return; }
                count -= 1;
            }
        }
    }

    if flags.has_misspell() {
        for response in MISSPELLS_TABLE {
            if text_lc.contains(response) {
                let reply = response.to_string() + " 🥹";
                catch_msg(msg.channel_id.say(&ctx.http, reply).await);

                if count == 0 { return; }
                count -= 1;
            }
        }
    }
}