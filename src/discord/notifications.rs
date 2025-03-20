
use serenity::{client::Context, model::id::ChannelId};

use crate::warframe::messenger::WarframeMessenger;

pub async fn send_weekly_notification(ctx: &Context, channel_id: ChannelId) {
    let embeds_results = vec![
        WarframeMessenger::embed_weekly_reset().await,
    ];

    for embed_result in embeds_results {
        match embed_result {
            Ok(embed) => {
                if let Err(err) = channel_id.send_message(&ctx.http, |m| m.set_embed(embed)).await {
                    eprintln!("Erreur en envoyant : {:?}", err);
                }
            }
            Err(err) => {
                eprintln!("Erreur dans la génération de l'embed : {:?}", err);
                let _ = channel_id.say(&ctx.http, "Erreur en envoyant.").await;
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
                    eprintln!("Erreur en envoyant l'embed : {:?}", err);
                }
            }
            Err(err) => {
                eprintln!("Erreur dans la génération de l'embed : {:?}", err);
                let _ = channel_id.say(&ctx.http, "Erreur dans la génération d'un des embeds, bordel.").await;
            }
        }
    }
}