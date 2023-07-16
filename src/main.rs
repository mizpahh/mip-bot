use std::{thread, time::Duration};

use once_cell::sync::Lazy;
use regex::Regex;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready, id::GuildId},
    prelude::GatewayIntents,
    Client,
};

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i:\bi['’]?m\s+)([\s\S]+)").unwrap());

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if let Some(caps) = RE.captures(&msg.content) {
            if let Some(mat) = caps.get(1) {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        ctx.http,
                        format!("# Hi {}, nice to meet you, I'm mip!", mat.as_str()),
                    )
                    .await
                {
                    eprintln!("{:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!(
            "ready {}#{}",
            data_about_bot.user.name, data_about_bot.user.discriminator
        );
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        println!("cache_ready");
        let mut guild_names = Vec::with_capacity(guilds.len());
        for guild_id in guilds {
            if let Some(name) = guild_id.name(&ctx) {
                guild_names.push(name);
            }
        }
        println!("guilds: {:?}", guild_names);
    }
}

async fn start_bot() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::builder(
        include_str!("credentials"),
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await?;
    client.start().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sleep_secs = 4;
    while let Err(e) = start_bot().await {
        println!("error starting bot: {:?}", e);
        thread::sleep(Duration::from_secs(sleep_secs));
        sleep_secs *= 2;
    }
    Ok(())
}
