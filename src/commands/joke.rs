use crate::types::types::{AppContext, Error};
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Get a joke from Jokes for Minecrafters
#[poise::command(slash_command)]
pub async fn joke(
    ctx: AppContext<'_>,
    #[description = "Joke number"] index: Option<usize>,
) -> Result<(), Error> {
    // get a random joke from the file jokes/alljokes.md or a specific one by index
    let joke = get_joke(index)?;

    ctx.say(joke).await?;
    Ok(())
}

fn get_joke(index: Option<usize>) -> Result<String, Error> {
    let path = Path::new("jokes/alljokes.md");
    let file = File::open(path).map_err(|e| Error::from(e))?;
    let reader = io::BufReader::new(file);

    let mut jokes = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| Error::from(e))?;
        if !line.trim().is_empty() {
            let processed_line = line.replace("\\n", "\n");
            jokes.push(processed_line);
        }
    }

    if jokes.is_empty() {
        return Err(Error::from("No jokes found"));
    }

    match index {
        Some(idx) => {
            if idx == 0 || idx > jokes.len() {
                return Err(Error::from(format!(
                    "Invalid joke number. Please use a number between 1 and {}",
                    jokes.len()
                )));
            }
            let joke_index = idx - 1; // Convert to 0-based index
            Ok(format!("-# #{}:\n{}", idx, jokes[joke_index].clone()))
        }
        None => {
            // Select a random joke
            let mut rng = rand::rng();
            let joke_index = rng.random_range(0..jokes.len());
            Ok(format!(
                "-# #{}:\n{}",
                joke_index + 1,
                jokes[joke_index].clone()
            ))
        }
    }
}
