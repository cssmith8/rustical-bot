use crate::types::Error;
use poise::serenity_prelude::model::id;

pub async fn edit_settings(id: id::UserId, key: String, value: String) -> Result<(), Error> {
    //get the user id
    let user_id = id;
    Ok(())
}
