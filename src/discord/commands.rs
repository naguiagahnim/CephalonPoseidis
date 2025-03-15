pub mod cycle_command {
    use serenity::{
        builder::CreateApplicationCommand,
        model::application::interaction::application_command::ApplicationCommandInteraction,
        prelude::*,
    };
    
    use crate::discord::notifications;
    
    pub fn register_mut(cmd: &mut CreateApplicationCommand) {
        cmd.name("cycles")
           .description("Affiche les informations sur les cycles de Warframe");
    }
    
    pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> String {
        notifications::send_cycles_notification(ctx, interaction.channel_id).await;
        "Les informations sur les cycles ont été envoyées.".to_string()
    }
}

pub mod weekly_reset_command {
    use serenity::{
        builder::CreateApplicationCommand,
        model::application::interaction::application_command::ApplicationCommandInteraction,
        prelude::*,
    };
    
    use crate::discord::notifications;
    
    pub fn register_mut(cmd: &mut CreateApplicationCommand) {
        cmd.name("weekly_reset")
           .description("Affiche les informations sur le reset hebdomadaire de Warframe");
    }
    
    pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> String {
        notifications::send_weekly_notification(ctx, interaction.channel_id).await;
        "Les informations sur le reset hebdomadaire ont été envoyées.".to_string()
    }
}



