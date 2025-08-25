use crate::{
    types::{
        translation::Translation,
        types::{AppContext, Error}
    },
    utils::translations::load_translations
};
use serenity::{
    all::CreateAttachment,
    builder::GetMessages
};

const NUM_MESSAGES: u8 = 3;

/// Translate recent messages in the channel
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
        if all_translations.len() > 50 {
            let mut output = String::new();
            for line in &all_translations {
                output.push_str(&line.to_string());
                output.push('\n');
            }

            let reply = poise::CreateReply::default().attachment(CreateAttachment::bytes(
                output.as_bytes(),
                "translations.txt",
            ));
            ctx.send(reply).await?;
        } else {
            let response = all_translations.join("\n");
            ctx.say(response).await?;
        }
    } else {
        ctx.say("No translations found.").await?;
    }

    Ok(())
}

fn test_for_translation(input: &str) -> Vec<String> {
    fn search(
        all_translations: &Vec<Translation>,
        input: &str,
        found_translations: &mut Vec<String>,
    ) {
        for translation in all_translations {
            if !input.contains(&translation.abbreviation) {
                continue;
            }
            let entry = translation.abbreviation.clone() + ": " + &translation.definition;
            if !found_translations.contains(&entry) {
                found_translations.push(entry.clone());
                search(
                    all_translations,
                    &translation.definition,
                    found_translations,
                );
            }
        }
    }

    let all = load_translations().unwrap_or_default();
    let mut found_translations = Vec::new();
    search(&all, input, &mut found_translations);
    found_translations
}
