use crate::types::translation::Translation;
use crate::types::types::{Data, Error};
use crate::utils::translations::save_translation;
use anyhow::Result;
use poise::serenity_prelude as serenity;
use regex::Regex;

pub async fn message(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
    new_message: &serenity::Message,
) -> Result<(), Error> {
    if new_message.author.bot {
        return Ok(());
    }
    let channel_id = new_message.channel_id;
    let content = new_message.content.to_lowercase();

    match content.as_str() {
        "rustical bot" => {
            let response = "I'm Rusting it";
            channel_id.say(&_ctx.http, response).await?;
            return Ok(());
        }
        _ => {}
    }

    test_for_translation(&content);

    Ok(())
}

fn test_for_translation(input: &str) {
    // Use regex to check if the message has text, then more text in parentheses
    let regex = Regex::new(r"(\w+)\s*\((.+)\)").unwrap();

    if let Some(captures) = regex.captures(&input) {
        // Create a new Translation struct where the first text is the abbreviation and the second text is the definition
        let translation = Translation {
            abbreviation: captures[1].to_string(),
            definition: captures[2].to_string(),
        };

        let definition = translation.definition.clone();

        if let Err(e) = save_translation(translation) {
            eprintln!("Error saving translation: {}", e);
        }

        test_for_translation(&definition);
    }
}
