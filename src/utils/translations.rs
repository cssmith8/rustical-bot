use crate::types::{translation::Translation, types::Error};
use crate::utils::db::create_or_open_db;
use anyhow::Result;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct DBTranslation {
    a: String,
    d: String,
}

pub fn save_translation(translation: Translation) -> Result<(), Error> {
    let db_path = "data/translations.db".to_string();
    let mut db = create_or_open_db(db_path);
    if !db.lexists("translations") {
        db.lcreate("translations")?;
    }
    db.ladd(
        "translations",
        &DBTranslation {
            a: translation.abbreviation,
            d: translation.definition,
        },
    )
    .ok_or_else(|| Error::from("Failed to add translation to database"))?;
    Ok(())
}

pub fn load_translations() -> Result<Vec<Translation>, Error> {
    let db_path = "data/translations.db".to_string();
    let db = create_or_open_db(db_path);

    let mut all_translations: Vec<Translation> = Vec::new();
    for item_iter in db.liter("translations") {
        let db_translation = item_iter.get_item::<DBTranslation>().unwrap();
        all_translations.push(Translation {
            abbreviation: db_translation.a,
            definition: db_translation.d,
        });
    }
    Ok(all_translations)
}
