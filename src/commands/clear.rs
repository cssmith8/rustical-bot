use crate::types::types::{AppContext, Error};
use anyhow::Result;
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Clear Commands"] // Struct name by default
pub struct ClearCommands {
    #[name = "Password"] // Field name by default
    #[max_length = 32]
    password: String,
}

/// Clear commands buttons
#[poise::command(slash_command)]
pub async fn clear(ctx: AppContext<'_>) -> Result<(), Error> {
    let data = ClearCommands::execute(ctx).await?;
    match data {
        Some(data) => {
            let clear_password = crate::utils::env::clear_password();
            if data.password == clear_password {
                poise::builtins::register_application_commands_buttons(ctx.into()).await?;
                ctx.say("Cleared all application commands.").await?;
            } else {
                ctx.say("Incorrect password. Commands not cleared.").await?;
            }
        }
        None => {
            return Ok(());
        }
    }

    Ok(())
}
