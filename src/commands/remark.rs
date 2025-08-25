use crate::types::types::{AppContext, Error};
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::utils::env;

/// Get a Joke for Minecrafters along with a snarky redditor remark
#[poise::command(slash_command)]
pub async fn remark(
    ctx: AppContext<'_>,
    #[description = "Joke number"] index: Option<usize>,
) -> Result<(), Error> {
    let joke = get_joke(index)?;
    ctx.say(joke).await?;
    Ok(())
}

fn get_joke(index: Option<usize>) -> Result<String, Error> {
    let full_path = env::static_path() + "jokes/jokesandremarks.md";
    let path = Path::new(&full_path);
    let file = File::open(path).map_err(|e| Error::from(e))?;
    let reader = io::BufReader::new(file);

    let mut all_lines = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| Error::from(e))?;
        if !line.trim().is_empty() {
            let processed_line = line.replace("\\n", "\n");
            all_lines.push(processed_line);
        }
    }

    // Group odd-numbered lines with their following lines
    let mut joke_pairs = Vec::new();
    for i in (0..all_lines.len()).step_by(2) {
        if i + 1 < all_lines.len() {
            joke_pairs.push((all_lines[i].clone(), all_lines[i + 1].clone()));
        }
    }

    if joke_pairs.is_empty() {
        return Err(Error::from("No joke pairs found"));
    }

    match index {
        Some(idx) => {
            if idx == 0 || idx > joke_pairs.len() {
                return Err(Error::from(format!(
                    "Invalid joke number. Please use a number between 1 and {}",
                    joke_pairs.len()
                )));
            }
            let pair_index = idx - 1; // Convert to 0-based index
            let (line1, line2) = &joke_pairs[pair_index];
            Ok(format!("-# #{}:\n{}\n```\n{}```", idx, line1, line2))
        }
        None => {
            // Select a random joke pair
            let mut rng = rand::rng();
            let pair_index = rng.random_range(0..joke_pairs.len());
            let (line1, line2) = &joke_pairs[pair_index];
            Ok(format!(
                "-# #{}:\n{}\n```\n{}```",
                pair_index + 1,
                line1,
                line2
            ))
        }
    }
}
