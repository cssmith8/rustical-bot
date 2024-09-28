use csv::{Reader, StringRecord, Writer};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use poise::serenity_prelude as serenity;
use poise::Modal;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use serenity::{
    model::prelude::{Message, Ready},
    Client,
};
use std::{env, io::Write};

struct Data {
    db: Mutex<PickleDb>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
type AppContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(Debug, serde::Deserialize)]
struct Record {
    name: String,
}

#[derive(Debug, Modal)]
#[name = "Epic Eggs"] // Struct name by default
struct MyModal {
    #[name = "First input label"] // Field name by default
    #[placeholder = "Your first input goes here"] // No placeholder by default
    #[min_length = 5] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 500]
    first_input: String,
    #[name = "Second input label"]
    #[paragraph] // Switches from single-line input to multiline text box
    second_input: Option<String>, // Option means optional input
}

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn say(ctx: Context<'_>, message: String) -> Result<(), Error> {
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn modal(ctx: AppContext<'_>) -> Result<(), Error> {
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

#[poise::command(slash_command, prefix_command)]
pub async fn paginate(ctx: Context<'_>) -> Result<(), Error> {
    let pages = &[
        "Content of first page",
        "Content of second page",
        "Content of third page",
        "Content of fourth page",
    ];

    //red csv file at ../data/cool.csv
    let csv = std::fs::read_to_string("./data/cool.csv").expect("Could not read file");
    //create a list of all lines of csv file
    let lines: Vec<&str> = csv.lines().collect();
    println!("{:?}", lines);

    poise::samples::paginate(ctx, pages).await?;

    Ok(())
}

//send a message in channel c
async fn rustical_message(ctx: &serenity::Context, data: &Data, c: ChannelId) -> Result<(), Error> {
    let mut db = data.db.lock().await;

    let index: i32 = db.get::<i32>("line").unwrap_or_default();
    db.set("line", &(index + 1)).unwrap();

    let mut rdr = csv::Reader::from_path("./data/cool.csv")?;
    let mut results: Vec<Record> = vec![];
    for result in rdr.deserialize::<Record>() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        match result {
            Ok(rec) => results.push(rec),
            Err(err) => {
                println!("ERORR PARSING: {}", err.to_string())
            }
        }
    }

    let message = match results.get((index % (results.len() as i32)) as usize) {
        Some(res) => res.name.clone(),
        None => "Couldn't get one lol".to_string(),
    };

    let channel = c;
    let channel = channel
        .to_channel(&ctx.http)
        .await
        .expect("this channel will always work");
    if let Some(channel) = channel.guild() {
        channel.say(&ctx.http, message + " :money_mouth:").await?;
    }
    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.tag());
            rustical_message(ctx, data, ChannelId::new(1160065321013620857)).await?;
        }
        // me when the
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }
            //not case sensitive
            if new_message.content.eq_ignore_ascii_case("rustical bot") {
                rustical_message(ctx, data, new_message.channel_id).await?;
            }
        }
        _ => {}
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected discord token env");

    let db = PickleDb::load(
        "data/real.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), modal(), paginate(), say()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db: Mutex::new(db?),
                })
            })
        })
        .build();

    let mut client = Client::builder(token, intents)
        //.event_handler(Handler {})
        .framework(framework)
        .await
        .expect("Could not create client");

    if let Err(e) = client.start().await {
        println!("Client error: {}", e.to_string());
    }
}
