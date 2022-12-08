use std::env;

use serenity::async_trait;
use serenity::client::EventHandler;
use serenity::model::prelude::*;
use serenity::prelude::*;
// use serenity::utils::token;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _context: Context, ready: Ready) {
        println!(
            "{} ready, watching {} guild(s).",
            ready.user.name,
            ready.guilds.len()
        );
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("TOKEN must be a set environment variable.");

    // broken
    // println!("isOk? {}", token::validate(&token).is_ok());

    let mut client = Client::builder(&token, GatewayIntents::default())
        .event_handler(Handler)
        .await
        .expect("Error occurred when setting event handlers.");

    client.start().await.expect("Error starting the client.");
}
