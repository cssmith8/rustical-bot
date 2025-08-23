use crate::types::types::{AppContext, Error};
use crate::utils::log::load_all_logs;

/// Output recent logs of bot activity
#[poise::command(slash_command)]
pub async fn logs(
    ctx: AppContext<'_>,
    #[description = "Count [1-50]"] count: Option<usize>,
) -> Result<(), Error> {
    let logs = load_all_logs()?;

    let logs_to_return: Vec<_> = {
        let count = count.unwrap_or(10).clamp(1, 50);
        logs.into_iter()
            .rev()
            .take(count)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    };

    // build a string from the logs
    let response = logs_to_return
        .into_iter()
        .map(|log| log.display())
        .collect::<Vec<_>>()
        .join("\n");

    ctx.say(response).await?;

    Ok(())
}
