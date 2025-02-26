use serenity::builder::CreateCommand;
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::discord::notifications;

pub mod cycle_command {
    use super::*;

    pub fn register() -> CreateCommand {
        CreateCommand::new(String::from("cycles"))
            .description("Affiche les informations sur les cycles de Warframe")
    }

    pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
        notifications::send_cycles_notification(ctx, interaction.channel_id).await;
        "Les informations sur les cycles ont été envoyées.".to_string()
    }
}

pub mod weekly_reset_command {
    use super::*;

    pub fn register() -> CreateCommand {
        CreateCommand::new(String::from("weekly_reset"))
            .description("Affiche les informations sur le reset hebdomadaire de Warframe")
    }

    pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> String {
        notifications::send_weekly_reset_notification(ctx, interaction.channel_id).await;
        "Les informations sur le reset hebdomadaire ont été envoyées.".to_string()
    }
}
