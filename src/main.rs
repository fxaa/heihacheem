mod tekken;
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate nom;
use serenity::{async_trait, framework::StandardFramework};
use serenity::{
    client::{Context, EventHandler},
    prelude::RwLock,
    Client,
};
use serenity::{framework::standard::Args, model::channel::Message};
use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    prelude::TypeMapKey,
};
use tekken::{
    config::{load_mokujin_config, MokujinMoveEntry},
    constants::find_char_name,
};

use std::{collections::HashMap, env::set_var, sync::Arc};

#[group]
#[commands(tk)]
struct Characters;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

struct CharacterMoveMap;
impl TypeMapKey for CharacterMoveMap {
    type Value = Arc<RwLock<HashMap<String, Vec<MokujinMoveEntry>>>>;
}

#[tokio::main]
async fn main() {
    set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&CHARACTERS_GROUP);

    // Login with a bot token from the environment
    let token = String::from("<redacted>");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    let char_moves = load_mokujin_config().expect("uh oh");
    {
        let mut data = client.data.write().await;

        data.insert::<CharacterMoveMap>(Arc::new(RwLock::new(char_moves)));
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn tk(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let char_name_input = args.current().unwrap().to_owned();
    let cleaned_char_name = find_char_name(char_name_input);
    let char_move_input = args.advance().current().unwrap();

    msg.reply(
        ctx,
        ctx.data
            .read()
            .await
            .get::<CharacterMoveMap>()
            .expect("shit, CharacterMoveMap")
            .read()
            .await
            .get(cleaned_char_name.as_str())
            .expect("shit, cleaned_char_name")
            .iter()
            .find(|char_move| char_move.command.eq(char_move_input))
            .unwrap()
            .clone()
            .damage
            .as_str(),
    )
    .await?;

    Ok(())
}
