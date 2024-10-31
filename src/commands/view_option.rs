use crate::types::{AppContext, Contract, Error};
use crate::types::Position;
use crate::utils::{get_position_status, open_option_db};
use chrono::Datelike;
use poise::serenity_prelude::{self as serenity, Colour};

const SELECT_TEXT: &str = "**Position Selected**\n> Use `/close` to close the position\n> Use `/roll` to roll the position\n> Use `/expire` if the option expired\n> Use `/assign` if the option was assigned\n> Use `/edit` to edit position info\n> Use `/date` to change open date";

pub struct OpenPosition {
    id: usize,
    pos: Position,
}

#[poise::command(slash_command)]
pub async fn view(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    //immutable db
    let db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };

    let mut open_positions: Vec<OpenPosition> = Vec::new();
    let mut id: usize = 0;
    // iterate over the items in list1
    for item_iter in db.liter("positions") {
        if get_position_status(item_iter.get_item::<Position>().unwrap()) == "open" {
            open_positions.push(OpenPosition {
                id: id,
                pos: item_iter.get_item::<Position>().unwrap(),
            });
        }
        id += 1;
    }

    //if no open options, return
    if open_positions.len() == 0 {
        ctx.say("You have no open positions").await?;
        return Ok(());
    }

    view_open(ctx, open_positions).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn details(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());
    let db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };
    let edit_id: i32 = db.get("edit_id").unwrap();
    if edit_id == -1 {
        ctx.say("No open position selected").await?;
        return Ok(());
    }
    if edit_id >= db.llen("positions") as i32 {
        ctx.say("Invalid selection").await?;
        return Ok(());
    }
    let position: Position = db.lget("positions", edit_id as usize).unwrap();

    view_contracts(ctx, &position.contracts).await?;

    Ok(())
}

pub async fn view_contracts(
    ctx: AppContext<'_>,
    contracts: &Vec<Contract>,
) -> Result<(), serenity::Error> {
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    let reply = {
        let mut components = vec![];
        if contracts.len() > 1 {
            components.push(serenity::CreateActionRow::Buttons(vec![
                serenity::CreateButton::new(&prev_button_id).emoji('◀'),
                serenity::CreateButton::new(&next_button_id).emoji('▶'),
            ]));
        }

        poise::CreateReply::default()
            .embed(
                serenity::CreateEmbed::default()
                    .description(stringify_contract(0, contracts.len() as u32, &contracts[0]).await),
            )
            .components(components)
    };

    ctx.send(reply).await?;

    if contracts.len() <= 1 {
        return Ok(());
    }

    let mut current_page = 0;
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        if press.user.id != ctx.interaction.user.id {
            ctx.say("Cannot view another user's contract").await?;
            continue;
        }

        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= contracts.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(contracts.len() - 1);
        } else {
            continue;
        }

        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new().embed(
                        serenity::CreateEmbed::new().description(
                            stringify_contract(
                                current_page as u32,
                                contracts.len() as u32,
                                &contracts[current_page],
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

pub async fn view_open(
    ctx: AppContext<'_>,
    pages: Vec<OpenPosition>,
) -> Result<(), serenity::Error> {
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
            .embed(
                serenity::CreateEmbed::default()
                    .description(stringify_position(0, pages.len() as u32, &pages[0]).await),
            )
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
        } else if press.data.custom_id == select_button_id {
            //MyModal::execute(ctx).await?;
            close_button(ctx, pages[current_page].id).await.unwrap();
            //ctx.say(SELECT_TEXT).await?;
            let reply = poise::CreateReply::default().embed(
                serenity::CreateEmbed::default()
                    .description(SELECT_TEXT)
                    .color(Colour::DARK_GREEN),
            );
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
                    serenity::CreateInteractionResponseMessage::new().embed(
                        serenity::CreateEmbed::new().description(
                            stringify_position(
                                current_page as u32,
                                pages.len() as u32,
                                &pages[current_page],
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

pub async fn stringify_position(index: u32, length: u32, position: &OpenPosition) -> String {
    let rolls = position.pos.num_rolls();
    let option = &position.pos.contracts[position.pos.contracts.len() - 1].open;
    let date: String = option.expiry.month().to_string()
        + "/"
        + &option.expiry.day().to_string()
        + "/"
        + &(option.expiry.year() % 100).to_string();
    let opendate: String = option.date.month().to_string()
        + "/"
        + &option.date.day().to_string()
        + "/"
        + &(option.date.year() % 100).to_string();
    //capitalize the open type first letter
    let open_type = option
        .open_type
        .chars()
        .next()
        .unwrap()
        .to_uppercase()
        .collect::<String>();
    let index_string = format!("{}/{}", index + 1, length);
    let rolls_string = if rolls > 0 {
        format!("-# *Rolled {} times*\n", rolls)
    } else {
        "".to_string()
    };
    let title_string = format!(
        "{} {} ${} {}",
        option.ticker, date, option.strike, open_type
    );
    let info_string = format!(
        "*Opened on {}*\nPremium: ${}\nQuantity: {}",
        opendate,
        position.pos.aggregate_premium(),
        option.quantity
    );
    return format!(
        "-# {index_string}\n# {title_string}\n{rolls_string}{info_string}\n"
    );
}

pub async fn stringify_contract(index: u32, length: u32, contract: &Contract) -> String {
    let open = &contract.open;
    let close = &contract.close;
    let open_date = format!("{}/{}/{}", open.date.month(), open.date.day(), open.date.year() % 100);
    let expiry_date = format!("{}/{}/{}", open.expiry.month(), open.expiry.day(), open.expiry.year() % 100);
    let close_info = match close {
        Some(c) => format!("Closed on {}/{}/{} with premium ${}", c.date.month(), c.date.day(), c.date.year() % 100, c.premium),
        None => "Still open".to_string(),
    };
    format!(
        "Contract {}/{}\n\
        {} {} ${} {}\n\
        Premium: ${}\n\
        Quantity: {}\n\
        Opened on: {}\n\
        Status: {}",
        index + 1, length, open.ticker, expiry_date, open.strike, open.open_type, open.premium, open.quantity, open_date, close_info
    )
}

pub async fn close_button(ctx: AppContext<'_>, index: usize) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    let mut db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };
    db.set("edit_id", &index).unwrap();
    Ok(())
}
