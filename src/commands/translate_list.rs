use crate::types::types::{Context, Error};
use crate::{types::translation::Translation, utils::translations::load_translations};

use anyhow::Result;

#[poise::command(slash_command, prefix_command)]
pub async fn translate_list(ctx: Context<'_>) -> Result<(), Error> {
    let all = load_translations().unwrap_or_default();
    let mut message = String::new();
    for translation in all {
        message.push_str(&translation.abbreviation);
        message.push_str(": ");
        message.push_str(&translation.definition);
        message.push('\n');
    }

    ctx.say(message).await?;
    Ok(())
}
