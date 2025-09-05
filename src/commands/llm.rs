use anyhow::Result;
use serenity::{
    builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseFollowup},
    model::application::CommandInteraction,
};
use serenity::all::{CommandDataOptionValue, CommandOptionType, CreateCommandOption};
use tracing::{debug, info, warn, error};

pub async fn run(ctx: &serenity::all::Context, interaction: &CommandInteraction) -> Result<()> {
    info!("Running command: 'llm'");

    interaction.defer(&ctx.http).await?;

    let message = match interaction.data.options.first() {
        Some(option) => {
            match &option.value {
                CommandDataOptionValue::String(msg) => msg.clone(),
                _ => {
                    error!("Message option is not a string");
                    "メッセージが取得できませんでした".to_string()
                }
            }
        }
        None => {
            error!("Cannot find message option");
            "メッセージが取得できませんでした".to_string()
        }
    };

    let response_content = format!("入力したテキスト : {}\n出力したテキスト : {}", message, "これはテキストメッセージです");

    let response = CreateInteractionResponseFollowup::new().content(response_content);
    interaction.create_followup(ctx, response).await?;

    Ok(())
}

pub fn register() -> CreateCommand{
    CreateCommand::new("llm").description("これは実験的なコマンドです")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "message", "llmに送信するユーザーメッセージ").required(true))
}