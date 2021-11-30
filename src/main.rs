use once_cell::sync::Lazy;
use regex::Regex;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    Client,
};

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i:\b(?:i'm|im)\s+)([\s\S]+)").unwrap());

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user_id().await {
            return;
        }
        if let Some(caps) = RE.captures(&msg.content) {
            if let Some(mat) = caps.get(1) {
                let _ = msg
                    .channel_id
                    .say(
                        ctx.http,
                        format!("Hi {}, nice to meet you, I'm mip!", mat.as_str()),
                    )
                    .await;
            }
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("ready");
    }

    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        println!("cache_ready");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::builder(std::env!("DISCORD_TOKEN"))
        .event_handler(Handler)
        .await?;
    client.start().await?;
    Ok(())
}
