use std::env;

use dotenv::dotenv;
use once_cell::sync::Lazy;
use serenity::{
    all::{CreateEmbed, CreateEmbedAuthor, CreateMessage, GatewayIntents},
    async_trait,
    model::{gateway::Ready, id, voice::VoiceState},
    prelude::*,
};
use tracing::{error, info};
use tracing_subscriber;

struct Handler;

static CHANNEL_ID: Lazy<u64> = Lazy::new(|| {
    let channel_id_str = env::var("DISCORD_CHANNEL_ID").expect("DISCORD_CHANNEL_ID must be set");
    channel_id_str.parse::<u64>().expect("Invalid channel ID")
});

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        old_state: Option<VoiceState>,
        new_state: VoiceState,
    ) {
        let member = match new_state.member {
            Some(member) => member,
            None => return,
        };

        let is_join_event = old_state.is_none() && new_state.channel_id.is_some();
        let is_leave_event = old_state.is_some() && new_state.channel_id.is_none();
        let is_switch_event = old_state.is_some()
            && new_state.channel_id.is_some()
            && old_state.clone().unwrap().channel_id != new_state.channel_id;

        if !is_join_event && !is_leave_event && !is_switch_event {
            return;
        }

        let log_channel_id = id::ChannelId::new(CHANNEL_ID.clone());
        let channel_id = if is_join_event || is_switch_event {
            new_state.channel_id.unwrap()
        } else {
            old_state.clone().unwrap().channel_id.unwrap()
        };

        let embed = if is_switch_event {
            CreateEmbed::new()
                .author(
                    CreateEmbedAuthor::new(member.user.name.clone()).icon_url(member.user.face()),
                )
                .description("switched voice channel")
                .field(
                    "From Channel",
                    format_channel_name(&ctx, old_state.clone().unwrap().channel_id.unwrap()).await,
                    false,
                )
                .field(
                    "To Channel",
                    format_channel_name(&ctx, new_state.channel_id.unwrap()).await,
                    false,
                )
                .field(
                    "User ID",
                    wrap_in_codeblock(member.user.id.to_string()),
                    false,
                )
                .fields(vec![
                    (
                        "From Channel ID",
                        wrap_in_codeblock(
                            old_state.clone().unwrap().channel_id.unwrap().to_string(),
                        ),
                        true,
                    ),
                    (
                        "To Channel ID",
                        wrap_in_codeblock(new_state.channel_id.unwrap().to_string()),
                        true,
                    ),
                ])
        } else {
            CreateEmbed::new()
                .author(
                    CreateEmbedAuthor::new(member.user.name.clone()).icon_url(member.user.face()),
                )
                .description(if is_join_event {
                    "joined voice channel"
                } else {
                    "left voice channel"
                })
                .field(
                    "Channel",
                    format_channel_name(&ctx, channel_id).await,
                    false,
                )
                .fields(vec![
                    (
                        "User ID",
                        wrap_in_codeblock(member.user.id.to_string()),
                        true,
                    ),
                    (
                        "Channel ID",
                        wrap_in_codeblock(log_channel_id.to_string()),
                        true,
                    ),
                ])
        };

        let message = CreateMessage::new().embed(embed);
        let result = log_channel_id.send_message(&ctx.http, message).await;
        if let Err(why) = result {
            error!("Error sending message: {:?}", why);
        }

        info!(
            "{} is {} {}",
            member.user.name,
            if is_join_event {
                "joining to channel"
            } else if is_leave_event {
                "leaving from channel"
            } else {
                "switching to channel"
            },
            channel_id.to_string(),
        );
    }
}

async fn format_channel_name(cache_http: impl CacheHttp, channel_id: id::ChannelId) -> String {
    format!(
        "{} ({})",
        channel_id.mention(),
        channel_id.name(cache_http).await.unwrap()
    )
}

fn wrap_in_codeblock(s: String) -> String {
    format!("```{}```", s)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Require a DISCORD_TOKEN environment variable");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MODERATION;

    tracing_subscriber::fmt::init();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }
}
