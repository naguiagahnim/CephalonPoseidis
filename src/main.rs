use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{
        application::interaction::Interaction, gateway::Ready, id::GuildId, application::interaction::InteractionResponseType
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

        let guild_pp_id = GuildId(config::get_guild_pp_id());

        let _ = guild_pp_id
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

            let guild_cl_id = GuildId(config::get_guild_cl_id());

            let _ = guild_cl_id
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

                let guild_di_id = GuildId(config::get_guild_di_id());

                let _ = guild_di_id
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

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Commande Slash reçue: {:#?}", command);
    
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                })
                .await
            {
                eprintln!("Putain d'erreur dans le defer : {:?}", why);
                return;
            }
    
            let content = match command.data.name.as_str() {
                "cycles" => commands::cycle_command::run(&ctx, &command).await,
                "weekly_reset" => commands::weekly_reset_command::run(&ctx, &command).await,
                _ => "Commande non implémentée.".to_string(),
            };
    
            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |resp| {
                    resp.content(content)
                })
                .await
            {
                eprintln!("Impossible de modifier la réponse : {:?}", why);
            }
        }
    }
    
}

#[tokio::main]
async fn main() {
    let token = config::get_token();
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token.as_str(), intents)
        .event_handler(Handler)
        .framework(StandardFramework::new())
        .await
        .expect("Erreur à la création du client");

    if let Err(why) = client.start().await {
        println!("Erreur du client: {:?}", why);
    }
}
