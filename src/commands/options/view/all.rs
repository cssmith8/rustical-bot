use crate::types::types::{AppContext, Error};
use crate::types::position::Position;
use crate::utils::{label_display, open_option_db};
use poise::serenity_prelude::{self as serenity};

#[poise::command(slash_command)]
pub async fn all(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    //immutable db
    let db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };

    let mut all_positions: Vec<Position> = Vec::new();
    // iterate over the items in list1
    for item_iter in db.liter("positions") {
        all_positions.push(item_iter.get_item::<Position>().unwrap());
    }

    //sort the open_positions vector by the expiry date of each position
    all_positions.sort_by_key(|pos| pos.get_final_contract().expiry());

    //if no open options, return
    if all_positions.len() == 0 {
        ctx.say("You have no open positions").await?;
        return Ok(());
    }

    view_all(ctx, all_positions).await?;

    Ok(())
}

async fn view_all(
    ctx: AppContext<'_>,
    pages: Vec<Position>,
) -> Result<(), serenity::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    // Send the embed with the first page as content
    let reply =
        {
            let components = serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new(&prev_button_id).emoji('◀'),
                serenity::CreateButton::new(&next_button_id).emoji('▶'),
            ]);

            poise::CreateReply::default()
                .embed(serenity::CreateEmbed::default().description(
                    label_display(
                                0,
                                pages.len() as u32,
                                &format!(
                                    "{}Status: `{}`",
                                    pages[0].display(),
                                    pages[0].get_status()
                                ),
                            ).await,
                ))
                .components(vec![components])
        };

    ctx.send(reply).await?;

    // Loop through incoming interactions with the navigation buttons
    let mut current_page = 0;
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        // We defined our button IDs to start with `ctx_id`. If they don't, some other command's
        // button was pressed
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        // Timeout when no navigation button has been pressed for 24 hours
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        if press.user.id != ctx.interaction.user.id {
            ctx.say("Cannot select another user's position").await?;
            continue;
        }
        // Depending on which button was pressed, go to next or previous page
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            // This is an unrelated button interaction
            continue;
        }

        // Update the message with the new page contents
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new().embed(
                        serenity::CreateEmbed::new().description(
                            label_display(
                                current_page as u32,
                                pages.len() as u32,
                                &format!(
                                    "{}Status: `{}`",
                                    pages[current_page].display(),
                                    pages[current_page].get_status()
                                ),
                            )
                            .await,
                        ),
                    ),
                ),
            )
            .await?;
    }

    Ok(())
}