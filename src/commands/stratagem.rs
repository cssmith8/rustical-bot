use crate::{
    types::types::{AppContext, Error},
    utils::db::create_or_open_db,
    utils::env,
};
use poise::serenity_prelude::{self as serenity};

/// Real gifs
#[poise::command(slash_command)]
pub async fn stratagem(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = env::data_path() + "stratagem.db";

    //immutable db
    let db = create_or_open_db(db_location);

    /*
    let mut open_positions: Vec<OpenPosition> = Vec::new();
    let mut id: usize = 0;
    // iterate over the items in list1
    for item_iter in db.liter("positions") {
        if item_iter.get_item::<Position>().unwrap().get_status() == "open" {
            open_positions.push(OpenPosition {
                id: id,
                pos: item_iter.get_item::<Position>().unwrap(),
            });
        }
        id += 1;
    }
    */

    view_open(ctx).await?;

    Ok(())
}

async fn view_open(ctx: AppContext<'_>) -> Result<(), serenity::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let left_button_id = format!("{}left", ctx_id);
    let up_button_id = format!("{}up", ctx_id);
    let down_button_id = format!("{}down", ctx_id);
    let right_button_id = format!("{}right", ctx_id);

    // Send the embed with the first page as content
    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&left_button_id).emoji('◀'),
            serenity::CreateButton::new(&up_button_id).label("up"),
            serenity::CreateButton::new(&down_button_id).label("down"),
            serenity::CreateButton::new(&right_button_id).emoji('▶'),
        ]);

        poise::CreateReply::default()
            .embed(serenity::CreateEmbed::default().description("Awaiting input..."))
            .components(vec![components])
    };

    ctx.send(reply).await?;

    // Loop through incoming interactions with the navigation buttons
    let mut current_path = String::new();
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        if press.user.id != ctx.interaction.user.id {
            ctx.say("Cannot use another user's input").await?;
            continue;
        }

        if press.data.custom_id == left_button_id {
            current_path += "a";
        } else if press.data.custom_id == right_button_id {
            current_path += "d";
        } else if press.data.custom_id == up_button_id {
            current_path += "w";
        } else if press.data.custom_id == down_button_id {
            current_path += "s";
        } else {
            // This is an unrelated button interaction
            continue;
        }

        /*
        } else if press.data.custom_id == select_button_id {
            //MyModal::execute(ctx).await?;
            select_button(ctx, pages[current_page].id).await.unwrap();
            //ctx.say(SELECT_TEXT).await?;
            let reply = poise::CreateReply::default().embed(
                serenity::CreateEmbed::default()
                    .description(SELECT_TEXT)
                    .color(Colour::DARK_GREEN),
            );
            ctx.send(reply).await?;
        }
        */

        // Update the message with the new page contents
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new().embed(
                        serenity::CreateEmbed::new()
                            .description(format!("current input: {}", current_path)),
                    ),
                ),
            )
            .await?;
    }

    Ok(())
}
