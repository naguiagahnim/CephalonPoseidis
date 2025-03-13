use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

use CephalonPoseidis::config;
use CephalonPoseidis::discord::commands;

struct Handler;

use serenity::
    framework::standard::
        StandardFramework
;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} est connecté !", ready.user.name);

        let guild_id = GuildId(config::get_guild_id().await);

        let _ = guild_id
            .set_application_commands(&ctx.http, |commands| {
                commands
                    .create_application_command(|cmd| {
                        commands::cycle_command::register_mut(cmd);
                        cmd
                    })
                    .create_application_command(|cmd| {
                        commands::weekly_reset_command::register_mut(cmd);
                        cmd
                    })
            })
            .await;

        println!("Commandes Slash enregistrées.");
    }
}

#[tokio::main]
async fn main() {
    let token = config::get_token();
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token.await.as_str(), intents)
        .event_handler(Handler)
        .framework(StandardFramework::new())
        .await
        .expect("Erreur à la création du client");

    if let Err(why) = client.start().await {
        println!("Erreur du client: {:?}", why);
    }
}
