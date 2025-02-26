use serenity::all::ChannelId;
use serenity::all::GuildId;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::application::Interaction;
use serenity::prelude::*;
use tokio::time::{self, Duration};
use CephalonPoseidis::config;
use CephalonPoseidis::discord::notifications;
use CephalonPoseidis::discord::commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "cycles" => commands::cycle_command::run(&ctx, &command).await,
                "weekly_reset" => commands::weekly_reset_command::run(&ctx, &command).await,
                _ => "Not implemented :(".to_string(),
            };

            if let Err(why) = command.create_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} est connecté !", ready.user.name);

        let guild_id = GuildId::new(config::get_guild_id().await);

        let commands = guild_id.set_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::cycle_command::register())
                .create_application_command(|command| commands::weekly_reset_command::register())
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);

        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            let channel_id = ChannelId::new(config::get_clan_cid().await);

            let now = time::Instant::now();
            let next_hour = now + Duration::from_secs(60 * (60 - (now.elapsed().as_secs() % 60)));
            time::sleep_until(next_hour).await;

            let mut interval = time::interval(Duration::from_secs(3600)); // 1 heure
            loop {
                interval.tick().await;

                notifications::send_weekly_reset_notification(&ctx_clone, channel_id).await;
                notifications::send_cycles_notification(&ctx_clone, channel_id).await;
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let token = config::get_token();
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token.await.as_str(), intents)
        .event_handler(Handler)
        .await
        .expect("Erreur à la création du client");

    if let Err(why) = client.start().await {
        println!("Erreur du client: {:?}", why);
    }
}
