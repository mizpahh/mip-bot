use once_cell::sync::Lazy;
use regex::Regex;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready, id::GuildId},
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
                        format!("Hi {}, nice to meet you, I'm mip!", mat.as_str()),
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
            if let Some(name) = guild_id.name(&ctx).await {
                guild_names.push(name);
            }
        }
        println!("guilds: {:?}", guild_names);
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
