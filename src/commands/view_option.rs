use crate::types::{AppContext, Error};
use crate::types::OptionOpen;
use chrono::Datelike;
use poise::serenity_prelude::{self as serenity, Colour};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

use super::option_settings::edit_settings;

const SELECT_TEXT: &str = "**Position Selected**\n> Use `/close` to close the position\n> Use `/roll` to roll the position\n> Use `/expire` if the option expired\n> Use `/assign` if the option was assigned\n> Use `/edit` to edit position info\n> Use `/date` to change open date";

#[poise::command(slash_command)]
pub async fn view(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;

    //open immutable db
    let db_location = format!("data/{}_open.db", userid.to_string());
    let opendb = match PickleDb::load(
        db_location.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
        Err(_e) => {
            ctx.say("No option history found").await?;
            return Ok(());
        }
    };

    let end_id = opendb.get::<u32>("end_id").unwrap();
    let mut open_options: Vec<OptionOpen> = Vec::new();
    for i in 0..end_id {
        let open_option = opendb.get::<OptionOpen>(i.to_string().as_str()).unwrap();
        //if the status is open, add to list
        if open_option.status == "open" {
            open_options.push(open_option);
        }
    }

    //if no open options, return
    if open_options.len() == 0 {
        ctx.say("You have no open positions").await?;
        return Ok(());
    }

    view_open(ctx, open_options).await?;

    Ok(())
}

pub async fn view_open(ctx: AppContext<'_>, pages: Vec<OptionOpen>) -> Result<(), serenity::Error> {
    // Define some unique identifiers for the navigation buttons
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let select_button_id = format!("{}select", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    // Send the embed with the first page as content
    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji('◀'),
            serenity::CreateButton::new(&select_button_id).label("Select"),
            serenity::CreateButton::new(&next_button_id).emoji('▶'),
        ]);

        poise::CreateReply::default()
            .embed(serenity::CreateEmbed::default().description(stringify(0, pages.len() as u32, &pages[0]).await))
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
        if (press.user.id != ctx.interaction.user.id) {
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
        } else if press.data.custom_id == select_button_id {
            //MyModal::execute(ctx).await?;
            close_button(ctx, pages[current_page].id).await.unwrap();
            //ctx.say(SELECT_TEXT).await?;
            let reply = poise::CreateReply::default().embed(serenity::CreateEmbed::default().description(SELECT_TEXT).color(Colour::DARK_GREEN));
            ctx.send(reply).await?;
        } else {
            // This is an unrelated button interaction
            continue;
        }

        // Update the message with the new page contents
        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(serenity::CreateEmbed::new().description(stringify(current_page as u32, pages.len() as u32, &pages[current_page]).await)),
                ),
            )
            .await?;
    }

    Ok(())
}

pub async fn stringify(index: u32, length: u32, option: &OptionOpen) -> String {
    let date: String = option.expiry.month().to_string() + "/" + &option.expiry.day().to_string() + "/" + &(option.expiry.year() % 100).to_string();
    let opendate: String = option.date.month().to_string() + "/" + &option.date.day().to_string() + "/" + &(option.date.year() % 100).to_string();
    //capitalize the open type first letter
    let open_type = option.open_type.chars().next().unwrap().to_uppercase().collect::<String>();
    let string = format!(
        "{}/{}\n# {} {} ${} {}\n*Opened on {}*\nPremium: ${}\nQuantity: {}\n",
        index + 1, length, option.ticker, date, option.strike, open_type, opendate, option.premium, option.quantity
    );
    string
}

pub async fn close_button(ctx: AppContext<'_>, index: u32) -> Result<(), Error> {
    edit_settings(ctx.interaction.user.id, "edit_id".to_string(), index.to_string()).await?;
    Ok(())
}