
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
    let embeds_results = vec![
        WarframeMessenger::embed_duviri().await,
        WarframeMessenger::embed_cetus().await,
        WarframeMessenger::embed_necralisk().await,
        WarframeMessenger::embed_vallis().await,
        WarframeMessenger::embed_zariman().await,
    ];

    for embed_result in embeds_results {
        match embed_result {
            Ok(embed) => {
                if let Err(err) = channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
                    eprintln!("Putain d'erreur en envoyant l'embed : {:?}", err);
                }
            }
            Err(err) => {
                eprintln!("Erreur dans la génération de l'embed : {:?}", err);
                let _ = channel_id.say(&ctx.http, "Erreur dans la génération d'un des embeds, bordel.").await;
            }
        }
    }
}


pub async fn send_duviri_embed(ctx: &Context, channel_id: ChannelId) {
    match WarframeMessenger::embed_duviri().await {
        Ok(embed) => {
            if let Err(why) = channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
                eprintln!("Erreur: {:?}", why);
            }
        }
        Err(err) => {
            eprintln!("Erreur : {:?}", err);
            if let Err(why) = channel_id.say(&ctx.http, "Impossible de générer le message").await {
                eprintln!("Erreur pendant l'envoi {:?}", why);
            }
        }
    }
}

pub async fn send_cetus_embed(ctx: &Context, channel_id: ChannelId) {
    match WarframeMessenger::embed_cetus().await {
        Ok(embed) => {
            if let Err(why) = channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
                eprintln!("Erreur en envoyant : {:?}", why);
            }
        }
        Err(err) => {
            eprintln!("Erreur lors de la génération : {:?}", err);
            let _ = channel_id.say(&ctx.http, "Impossible de générer l’embed Cetus").await;
        }
    }
}

pub async fn send_necralisk_embed(ctx: &Context, channel_id: ChannelId) {
    match WarframeMessenger::embed_necralisk().await {
        Ok(embed) => {
            if let Err(why) = channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
                eprintln!("Erreur pendant l’envoi de l’embed Necralisk: {:?}", why);
            }
        }
        Err(err) => {
            eprintln!("Impossible de générer l’embed Necralisk: {:?}", err);
            let _ = channel_id.say(&ctx.http, "L'envoie a raté.").await;
        }
    }
}

pub async fn send_vallis_embed(ctx: &Context, channel_id: ChannelId) {
    match WarframeMessenger::embed_vallis().await {
        Ok(embed) => {
            if let Err(why) = channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
                eprintln!("Erreur lors de l’envoi de l’embed Vallis: {:?}", why);
            }
        }
        Err(err) => {
            eprintln!("Erreur de génération de l’embed Vallis: {:?}", err);
            let _ = channel_id.say(&ctx.http, "Embed Vallis n'a pas pu s'envoyer.").await;
        }
    }
}

pub async fn send_zariman_embed(ctx: &Context, channel_id: ChannelId) {
    match WarframeMessenger::embed_zariman().await {
        Ok(embed) => {
            if let Err(why) = channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
                eprintln!("Erreur à l’envoi de l’embed Zariman: {:?}", why);
            }
        }
        Err(err) => {
            eprintln!("Erreur Zariman: {:?}", err);
            let _ = channel_id.say(&ctx.http, "Embed Zariman indisponible.").await;
        }
    }
}
