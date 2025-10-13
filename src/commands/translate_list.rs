use crate::{
    types::{
        types::{Context, Error},
    },
    utils::translations::load_translations,
};
use anyhow::Result;

#[poise::command(slash_command, prefix_command)]
pub async fn translate_list(ctx: Context<'_>) -> Result<(), Error> {
    let all = load_translations().unwrap_or_default();
    let mut message = String::new();
    for translation in all {
        message.push_str(&format!("{}: {}\n", translation.abbreviation, translation.definition));
    }

    ctx.say(message).await?;
    Ok(())
}
