use crate::types::{AppContext, Error};

#[derive(Debug, serde::Deserialize)]
struct BrawlerRaw {
    //id: i8,
    //name: String,
}

struct Brawler {
    //id: i8,
    //name: String,
    matchups: Vec<f32>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Matchup {
    id0: f32,
    id1: f32,
    id2: f32,
    id3: f32,
    id4: f32,
    id5: f32,
    id6: f32,
    id7: f32,
    id8: f32,
    id9: f32,
    id10: f32,
    id11: f32,
    id12: f32,
    id13: f32,
    id14: f32,
    id15: f32,
}

pub async fn get_matchup(id1: i8, id2: i8) -> Result<f32, Error> {
    let mut rdr = csv::Reader::from_path("./data/stars/brawlers.csv")?;
    let mut brawlers_raw: Vec<BrawlerRaw> = vec![];
    for result in rdr.deserialize::<BrawlerRaw>() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        match result {
            Ok(rec) => brawlers_raw.push(rec),
            Err(err) => {
                println!("ERORR PARSING: {}", err.to_string())
            }
        }
    }

    let mut brawlers: Vec<Brawler> = vec![];

    let mut invert = false;
    let mut firstid = id1;
    let mut secondid = id2;

    if id1 == id2 {
        //if the ids are equal, respond with -1
        //ctx.say("-1").await?;
        return Ok(-1.0);
    } else if id1 < 0
        || id1 >= brawlers_raw.len() as i8
        || id2 < 0
        || id2 >= brawlers_raw.len() as i8
    {
        //if the ids are out of bounds, respond
        //println!("Invalid IDs: {} {}", id1, id2);
        //ctx.say("Invalid IDs").await?;
        return Ok(-2.0);
    } else if id1 < id2 {
        //if the ids are in the wrong order, swap them
        firstid = id2;
        secondid = id1;
        invert = true;
    }

    //read the matchups csv file
    rdr = csv::Reader::from_path("./data/stars/matchups.csv")?;
    let mut matchups: Vec<Matchup> = vec![];
    for result in rdr.deserialize::<Matchup>() {
        match result {
            Ok(rec) => {
                let rec_clone = rec.clone();
                matchups.push(rec);
                let mut matchup: Vec<f32> = vec![];
                matchup.push(rec_clone.id0);
                matchup.push(rec_clone.id1);
                matchup.push(rec_clone.id2);
                matchup.push(rec_clone.id3);
                matchup.push(rec_clone.id4);
                matchup.push(rec_clone.id5);
                matchup.push(rec_clone.id6);
                matchup.push(rec_clone.id7);
                matchup.push(rec_clone.id8);
                matchup.push(rec_clone.id9);
                matchup.push(rec_clone.id10);
                matchup.push(rec_clone.id11);
                matchup.push(rec_clone.id12);
                matchup.push(rec_clone.id13);
                matchup.push(rec_clone.id14);
                matchup.push(rec_clone.id15);

                //store the vector in brawler
                brawlers.push(Brawler {
                    // id: (matchups.len() - 1) as i8,
                    // name: brawlers_raw
                    //     .get(matchups.len() - 1 as i8 as usize)
                    //     .unwrap()
                    //     .name
                    //     .clone(),
                    matchups: matchup,
                });
            }
            Err(err) => {
                println!("ERORR PARSING: {}", err.to_string())
            }
        }
    }

    let mut message: f32 = match brawlers.get(firstid as usize) {
        Some(res) => res.matchups.get(secondid as usize).unwrap().clone(),
        None => "-3".parse().unwrap(),
    };
    if invert && message > 0.0 {
        message = 1.0 - message;
    }
    return Ok(message);
}

//get team matchup
pub async fn get_team_matchup(blue: Vec<i8>, red: Vec<i8>) -> Result<f32, Error> {
    let mut total = 0.0;
    //for each brawler on the blue team
    for i in 0..blue.len() {
        //for each brawler on the red team
        for j in 0..red.len() {
            //get the matchup between the two brawlers
            let matchup = get_matchup(blue[i], red[j]).await?;
            //add the matchup to the total
            total += matchup;
            total -= 0.5;
        }
    }
    return Ok(total);
}

#[poise::command(slash_command)]
pub async fn matchup(ctx: AppContext<'_>, id1: i8, id2: i8) -> Result<(), Error> {
    let message = get_matchup(id1, id2).await?;
    //respond with the number of brawlers
    match message {
        -1.0 => ctx.say("Error: Same Brawler").await?,
        -2.0 => ctx.say("Error: Invalid Brawler").await?,
        -3.0 => ctx.say("Error: Parsing Error").await?,
        _ => ctx.say(format!("{}", message)).await?,
    };
    //get_team_matchup with 2 empty vectors
    get_team_matchup(vec![], vec![]).await?;
    Ok(())
}
