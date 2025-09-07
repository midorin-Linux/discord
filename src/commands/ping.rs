use anyhow::{Context, Result};
use serenity::{
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage},
    model::application::CommandInteraction,
};
use std::time::Instant;
use tracing::{debug, info, instrument};

#[instrument(skip(ctx, command))]
pub async fn run(ctx: &serenity::all::Context, command: &CommandInteraction) -> Result<()> {
    info!("Running command: 'ping'");

    let data = CreateInteractionResponseMessage::new().content("pong!");
    let builder = CreateInteractionResponse::Message(data);

    command
        .create_response(&ctx.http, builder)
        .await
        .context("Failed to create interaction response")?;

    debug!("Successfully responded to ping command");

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("pingを返します")
}