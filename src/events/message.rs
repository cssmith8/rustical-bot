use crate::{
    types::{
        translation::Translation,
        types::{Data, Error},
    },
    utils::{
        bot::send_message_in_channel,
        log::log,
        translations::{get_translation, save_translation},
    },
};
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

    let _ = test_for_translation(&content);

    handle_echo(&content);

    Ok(())
}

fn test_for_translation(input: &str) -> Option<String> {
    // Use regex to check if the message has text, then more text in parentheses
    let regex = Regex::new(r"^([^\(]+)\s*\((.+)\)").unwrap();

    if let Some(captures) = regex.captures(&input) {
        // Create a new Translation struct where the first text is the abbreviation and the second text is the definition
        let mut translation = Translation {
            abbreviation: captures[1].trim().to_string(),
            definition: captures[2].trim().to_string(),
        };

        if let Some(t) = test_for_translation(&translation.definition) {
            translation.definition = t;
        }

        match get_translation(&translation.abbreviation) {
            Ok(Some(_t)) => {
                log(format!(
                    "Translation already exists: {} -> {}",
                    translation.abbreviation, translation.definition
                ));
            }
            Ok(None) => {
                log(format!(
                    "Saving translation: {} -> {}",
                    translation.abbreviation, translation.definition
                ));
                if let Err(e) = save_translation(&translation) {
                    log(format!("Error saving translation: {}", e));
                }
            }
            Err(e) => {
                log(format!("Error getting translation: {}", e));
            }
        }

        return Some(translation.abbreviation);
    }
    None
}

fn handle_echo(input: &str) {
    let echo_regex = Regex::new(r#"^echo ?['"](.+)['"] ?>> ?['"]?<\#(\d+)>['"]?$"#).unwrap();
    if let Some(captures) = echo_regex.captures(input) {
        let message = captures.get(1).map_or("", |m| m.as_str());
        let channel_id = captures
            .get(2)
            .map_or("", |m| m.as_str())
            .parse::<u64>()
            .unwrap_or(0);
        send_message_in_channel(message, channel_id);
    }
}
