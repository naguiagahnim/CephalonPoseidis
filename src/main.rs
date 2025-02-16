use load_dotenv::load_dotenv;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} est connecté !", ready.user.name);
        
        if let Some(guild) = ready.guilds.first() {
            if let Some(channel) = guild.id.channels(&ctx.http).await.unwrap().values().next() {
                if let Err(why) = channel.id.say(&ctx.http, "Test").await {
                    println!("Erreur d'envoi de message: {:?}", why);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    load_dotenv!();
    let token = env!("DISCORD_TOKEN");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Erreur à la création du client");

    if let Err(why) = client.start().await {
        println!("Erreur du client: {:?}", why);
    }
}
