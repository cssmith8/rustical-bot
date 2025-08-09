use crate::types::types::{AppContext, Error};
use crate::utils::translations::load_translations;
use serenity::builder::GetMessages;

const NUM_MESSAGES: u8 = 1;

#[poise::command(slash_command)]
pub async fn translate(ctx: AppContext<'_>) -> Result<(), Error> {
    //print the 3 most recent messages in the channel to the console
    let channel_id = ctx.channel_id();

    let builder = GetMessages::new().limit(NUM_MESSAGES);
    let messages = channel_id.messages(ctx.http(), builder).await?;

    let mut all_translations = Vec::new();
    for message in messages.iter() {
        all_translations.extend(test_for_translation(&message.content));
    }

    if !all_translations.is_empty() {
        let response = all_translations.join("\n");
        ctx.say(response).await?;
    } else {
        ctx.say("No translations found.").await?;
    }
    Ok(())
}

fn test_for_translation(input: &str) -> Vec<String> {
    let all_translations = load_translations().unwrap_or_default();
    let mut found_translations = Vec::new();
    for translation in all_translations {
        if input.contains(&translation.abbreviation) {
            found_translations
                .push(translation.abbreviation.clone() + ": " + &translation.definition);
        }
    }
    found_translations
}
