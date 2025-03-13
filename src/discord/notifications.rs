
use serenity::{client::Context, model::id::ChannelId};

use crate::warframe::messenger::WarframeMessenger;

pub async fn send_weekly_reset_notification(ctx: &Context, channel_id: ChannelId) {
    let message_result = WarframeMessenger::announce_weekly_reset().await;

    match message_result {
        Ok(message) => {
            if let Err(why) = channel_id.say(&ctx.http, &message).await {
                eprintln!("Erreur lors de l'envoi du message: {:?}", why);
            }
        }
        Err(err) => {
            eprintln!("Erreur lors de la récupération des informations de réinitialisation hebdomadaire: {:?}", err);
            if let Err(why) = channel_id.say(&ctx.http, "Erreur lors de la récupération des informations de réinitialisation hebdomadaire. Veuillez réessayer plus tard.").await {
                eprintln!("Erreur lors de l'envoi du message d'erreur: {:?}", why);
            }
        }
    }
}

pub async fn send_cycles_notification(ctx: &Context, channel_id: ChannelId) {
    let message_result = WarframeMessenger::announce_cycles().await;

    match message_result {
        Ok(message) => {
            if let Err(why) = channel_id.say(&ctx.http, &message).await {
                eprintln!("Erreur lors de l'envoi du message: {:?}", why);
            }
        }
        Err(err) => {
            eprintln!("Erreur lors de la récupération des informations de cycles: {:?}", err);
            if let Err(why) = channel_id.say(&ctx.http, "Erreur lors de la récupération des informations de cycles. Veuillez réessayer plus tard.").await {
                eprintln!("Erreur lors de l'envoi du message d'erreur: {:?}", why);
            }
        }
    }
}