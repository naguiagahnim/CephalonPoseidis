use load_dotenv::load_dotenv;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use CephalonPoseidis::config;
use CephalonPoseidis::warframe::{WorldState, WeeklyReset, Invasions};
use CephalonPoseidis::discord::{commands, notifications};


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} est connecté !", ready.user.name);
    }
}

/*TODO config tâches périodiques dans main avec tokio::spawn du type : 
tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600)); // 1 heure
        loop {
            interval.tick().await;
            exemple ! : WorldState::update().await;
            Invasions::check().await;
*/

#[tokio::main]
async fn main() {
    load_dotenv!();
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
