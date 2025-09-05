use crate::{database, Config};
use crate::commands;

use anyhow::{Context, Result};
use serenity::{
    all::Context as SerenityContext,
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::EventHandler,
    model::{
        channel::Message,
        event::ResumedEvent,
        gateway::Ready,
        guild::Member,
        id::{GuildId, UserId},
        user::User,
    },
    prelude::*,
};
use serenity::all::Interaction;
use std::path::Path;
use sqlx::{Sqlite, SqlitePool};
use tracing::{debug, info, warn, error, instrument};

pub struct Handler {
    guild_id: u64,
    pool: SqlitePool,
}

impl Handler {
    pub async fn new(config: Config) -> Result<Self> {
        debug!("Initializing handler...");

        let guild_id = config.guild_id.parse::<u64>().context("Invalid guild ID")?;

        let pool = database::create_pool(config.database_url.as_str()).await.context("Failed to create database pool")?;

        debug!("Database schema created");

        debug!("Handler initialized");

        Ok(Self {
            guild_id,
            pool,
        })
    }
}

#[async_trait]
impl EventHandler for Handler {
    #[instrument(skip(self, ctx, msg), fields(
        channel_id = %msg.channel_id,
        user_id = %msg.author.id,
        content = %msg.content
    ))]
    async fn message(&self, ctx: serenity::all::Context, msg: Message) {
        if msg.author.bot {
            return;
        }

    }

    #[instrument(skip(self, ctx, ready), fields(user_id = %ready.user.id, user_name = %ready.user.name))]
    async fn ready(&self, ctx: SerenityContext, ready: Ready) {
        info!("{} is connected to Discord!", ready.user.name);

        let commands = GuildId::new(self.guild_id)
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
            ]).await;

        info!("Registered commands: {:?}", commands);
        info!("Ready!");
    }

    #[instrument(skip(self, _ctx, _resume))]
    async fn resume(&self, _ctx: SerenityContext, _resume: ResumedEvent) {
        info!("Resumed connection to Discord");
    }

    #[instrument(skip(self, ctx, interaction))]
    async fn interaction_create(&self, ctx: serenity::all::Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let options = command.data.options();
            let options_len = options.len();

            info!("Received command: {:?}", command.data.name);
            debug!(user_id = %command.user.id, channel_id = %command.channel_id, options_len, "Processing command interaction");

            if let Err(why) = match command.data.name.as_str() {
                "ping" => {
                    commands::ping::run(&ctx, &command).await
                },
                _ => {
                    warn!("Unknown command: {}", command.data.name);
                    let data = CreateInteractionResponseMessage::new().content("不明なコマンドです");
                    let builder = CreateInteractionResponse::Message(data);
                    command.create_response(&ctx.http, builder).await.map_err(|e| anyhow::anyhow!(e))
                },
            } {
                error!("Error during command execution: {:?}", why);
            }
        } else {
            debug!("Received non-command interaction; ignoring");
        }
    }
}