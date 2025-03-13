use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{
        gateway::Ready,
        id::{GuildId, ChannelId},
    },
    prelude::*,
};

use CephalonPoseidis::config;
use CephalonPoseidis::discord::{commands, notifications};

struct Handler;

use serenity::{
    framework::standard::
        StandardFramework
    ,
    model::interactions::{
            Interaction, InteractionResponseType,
        },
};

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} est connecté !", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "cycles" => commands::cycle_command::run(&ctx, &command).await,
                "weekly_reset" => commands::weekly_reset_command::run(&ctx, &command).await,
                _ => "Commande non implémentée".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(content)
                        })
                })
                .await
            {
                println!("Impossible de répondre à la commande : {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = config::get_token();
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token.await.as_str(), intents)
        .event_handler(Handler)
        .framework(StandardFramework::new()) // Ajout du framework ici
        .await
        .expect("Erreur à la création du client");

    if let Err(why) = client.start().await {
        println!("Erreur du client: {:?}", why);
    }
}
