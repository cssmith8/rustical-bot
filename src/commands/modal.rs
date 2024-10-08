use crate::types::{AppContext, Error};
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Epic Eggs"] // Struct name by default
pub struct MyModal {
    #[name = "Thing 1"] // Field name by default
    #[placeholder = "1"] // No placeholder by default
    #[min_length = 5] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 500]
    #[paragraph] // Switches from single-line input to multiline text box
    first_input: String,
    #[name = "Thing 2"]
    #[placeholder = "2"]
    second_input: Option<String>, // Option means optional input
}

#[poise::command(slash_command)]
pub async fn modal(ctx: AppContext<'_>) -> Result<(), Error> {
    //get the user id
    let data = MyModal::execute(ctx).await?;
    println!("Got data: {:?}", data);
    //use the data here
    match data {
        Some(data) => {
            // handle data
            data.first_input;
        }
        None => return Ok(()),
    }
    Ok(())
}
