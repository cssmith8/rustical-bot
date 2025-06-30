use poise::serenity_prelude::{self as serenity};
use crate::types::positionmonth::PositionMonth;
use crate::types::tradingmonth::TradingMonth;
use crate::types::types::{AppContext, Error};
use crate::types::position::Position;
use crate::utils::{label_display, open_option_db};
use std::collections::HashMap;

#[poise::command(slash_command)]
pub async fn month(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    let db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };

    let mut closed_positions: Vec<Position> = Vec::new();
    for item_iter in db.liter("positions") {
        let pos = item_iter.get_item::<Position>().unwrap();
        if pos.is_closed() {
            closed_positions.push(pos.clone());
        }
    }

    let mut profitmonths: Vec<PositionMonth> = Vec::new();
    let mut taxablemonths: Vec<PositionMonth> = Vec::new();

    for pos in &closed_positions {
        profitmonths.extend(pos.generate_distributed_months());
        taxablemonths.extend(pos.generate_taxable_months());
    }

    // new hashmap that stores strings -> TradingMonth
    let mut trading_months: HashMap<String, TradingMonth> = HashMap::new();

    // for each profitmonth:
    for profitmonth in &profitmonths {
        // create a unique id for the trading month, e.g., "YYYY-MM"
        let key = profitmonth.id();

        // if there does not exist a trading month with matching id, make one in the hashmap
        trading_months
            .entry(key.clone())
            .and_modify(|tm| {
                // else use the .combine() method on the tradingmonth to add the data of the profitmonth
                tm.combine(profitmonth.clone());
            })
            .or_insert_with(|| TradingMonth {
                year: profitmonth.year,
                month: profitmonth.month,
                gain: profitmonth.gain,
                investment: profitmonth.investment
            });
    }

    let mut taxable_trading_months: HashMap<String, TradingMonth> = HashMap::new();

    for profitmonth in &taxablemonths {
        // create a unique id for the trading month, e.g., "YYYY-MM"
        let key = profitmonth.id();

        // if there does not exist a trading month with matching id, make one in the hashmap
        taxable_trading_months
            .entry(key.clone())
            .and_modify(|tm| {
                // else use the .combine() method on the tradingmonth to add the data of the profitmonth
                tm.combine(profitmonth.clone());
            })
            .or_insert_with(|| TradingMonth {
                year: profitmonth.year,
                month: profitmonth.month,
                gain: profitmonth.gain,
                investment: profitmonth.investment
            });
    }

    let mut responses: Vec<String> = Vec::new();

    let mut chrono_returns: String = "**Daily Return Rate**\n-# Chronological Order\n\n".to_string();
    let mut chrono_gains: String = "**Distributed Gain**\n-# Chronological Order\n\n".to_string();
    let mut months_chrono: Vec<&TradingMonth> = trading_months.values().collect();
    months_chrono.sort_by(|a, b| b.id().partial_cmp(&a.id()).unwrap_or(std::cmp::Ordering::Equal));
    for tradingmonth in months_chrono {
        chrono_returns.push_str(&format!("{}\n", tradingmonth.display_daily_return_rate()));
        chrono_gains.push_str(&format!("{}\n", tradingmonth.display_distributed_gain()));
    }

    let mut returns_returns = "**Daily Return Rate**\n-# by Daily Return Rate\n\n".to_string();
    let mut months_returns: Vec<&TradingMonth> = trading_months.values().collect();
    months_returns.sort_by(|a, b| b.daily_return_rate().partial_cmp(&&a.daily_return_rate()).unwrap_or(std::cmp::Ordering::Equal));
    for tradingmonth in months_returns {
        returns_returns.push_str(&format!("{}\n", tradingmonth.display_daily_return_rate()));
    }

    let mut gains_gains = "**Distributed Gain**\n-# by Distributed Gain\n\n".to_string();
    let mut months_gains: Vec<&TradingMonth> = trading_months.values().collect();
    months_gains.sort_by(|a, b| b.gain.partial_cmp(&a.gain).unwrap_or(std::cmp::Ordering::Equal));
    for tradingmonth in months_gains {
        gains_gains.push_str(&format!("{}\n", tradingmonth.display_distributed_gain()));
    }

    let mut taxgains_chrono: String = "**Taxable Gain**\n-# Chronological Order\n\n".to_string();
    let mut tax_months_chrono: Vec<&TradingMonth> = taxable_trading_months.values().collect();
    tax_months_chrono.sort_by(|a, b| b.id().partial_cmp(&a.id()).unwrap_or(std::cmp::Ordering::Equal));
    for tradingmonth in tax_months_chrono {
        taxgains_chrono.push_str(&format!("{}\n", tradingmonth.display_distributed_gain()));
    }

    let mut taxgains_taxgains: String = "**Taxable Gain**\n-# by Taxable Gain\n\n".to_string();
    let mut tax_months_gains: Vec<&TradingMonth> = taxable_trading_months.values().collect();
    tax_months_gains.sort_by(|a, b| b.gain.partial_cmp(&a.gain).unwrap_or(std::cmp::Ordering::Equal));
    for tradingmonth in tax_months_gains {
        taxgains_taxgains.push_str(&format!("{}\n", tradingmonth.display_distributed_gain()));
    }

    responses.push(chrono_returns);
    responses.push(returns_returns);
    responses.push(chrono_gains);
    responses.push(gains_gains);
    responses.push(taxgains_chrono);
    responses.push(taxgains_taxgains);
    
    view_strings(ctx, responses).await?;
    Ok(())
}

async fn view_strings(
    ctx: AppContext<'_>,
    pages: Vec<String>,
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
                                    "{}",
                                    pages[0]
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
                                    "{}",
                                    pages[current_page]
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