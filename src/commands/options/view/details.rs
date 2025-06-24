use crate::types::{AppContext, Contract, Error, Position};
use crate::utils::{open_option_db};
use poise::serenity_prelude::{self as serenity};

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
            .embed(serenity::CreateEmbed::default().description(format!(
                "Contract {}/{}\n{}",
                1,
                contracts.len(),
                &contracts[0].display().await
            )))
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
                        serenity::CreateEmbed::new().description(format!(
                            "Contract {}/{}\n{}",
                            current_page + 1,
                            contracts.len(),
                            &contracts[current_page].display().await
                        )),
                    ),
                ),
            )
            .await?;
    }

    Ok(())
}