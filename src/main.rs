use std::env;

use serenity::async_trait;
use serenity::client::EventHandler;
use serenity::model::application::interaction::{
    Interaction, InteractionResponseType, InteractionType,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
// use serenity::utils::token;

mod application_commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        command::Command::create_global_application_command(&context, |command| {
            application_commands::ping::register(command)
        })
        .await
        .expect("An error occurred creating global application commands.");

        println!(
            "{} ready, watching {} guild(s).",
            ready.user.name,
            ready.guilds.len()
        );
    }

    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if interaction.kind().ne(&InteractionType::ApplicationCommand) {
            return;
        }

        let application_command = interaction.application_command().unwrap();

        let rename_me = match application_command.data.name.as_str() {
            "ping" => application_commands::ping::run(&application_command.data.options),
            _ => "NotImplemented".to_string(),
        };

        application_command
            .create_interaction_response(&context.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(rename_me))
            })
            .await
            .expect("An error occurred responding to an application command.");
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
