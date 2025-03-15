use super::api::WarframeApi;
use std::{collections::HashMap, error::Error};
use once_cell::sync::Lazy;
use serenity::{builder::CreateEmbed, utils::Colour};

pub struct WarframeMessenger;

static EMOTION_TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    [
        ("envy", "Envie"),
        ("joy", "Joie"),
        ("anger", "Colère"),
        ("sorrow", "Chagrin"),
        ("fear", "Peur"),
    ].iter().cloned().collect()
});

static CETUS_TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    [
        ("day", "jour"),
        ("night", "nuit"),
    ].iter().cloned().collect()
});

static NECRALISK_TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    [
        ("vome", "Vome"),
        ("fass", "Fass"),
    ].iter().cloned().collect()
});

static VALLIS_TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    [
        ("warm", "chaud"),
        ("cold", "froid"),
    ].iter().cloned().collect()
});

static FACTION_TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    [
        ("corpus", "Corpus"),
        ("grineer", "Grineer"),
    ].iter().cloned().collect()
});

impl WarframeMessenger {
    pub fn translate_emotion(emotion: &Option<String>) -> &str {
        emotion
            .as_deref()
            .and_then(|e| EMOTION_TRANSLATIONS.get(e))
            .map(|&s| s)
            .unwrap_or("Émotion inconnue")
    }

    pub fn translate_cetus(cetus: &Option<String>) -> &str {
        cetus
            .as_deref()
            .and_then(|e| CETUS_TRANSLATIONS.get(e))
            .map(|&s| s)
            .unwrap_or("Cycle Cetus inconnu")
    }

    pub fn translate_necralisk(cetus: &Option<String>) -> &str {
        cetus
            .as_deref()
            .and_then(|e| NECRALISK_TRANSLATIONS.get(e))
            .map(|&s| s)
            .unwrap_or("Créature de Deimos inconnue")
    }

    pub fn translate_vallis(vallis: &Option<String>) -> &str {
        vallis
            .as_deref()
            .and_then(|e| VALLIS_TRANSLATIONS.get(e))
            .map(|&s| s)
            .unwrap_or("Température inconnue")
    }

    pub fn translate_zariman(zariman: &Option<String>) -> &str {
        zariman
            .as_deref()
            .and_then(|e| FACTION_TRANSLATIONS.get(e))
            .map(|&s| s)
            .unwrap_or("Faction inconnue")
    }

    pub async fn announce_weekly_reset() -> Result<String, Box<dyn Error + Send + Sync>> {
        let worldstate = WarframeApi::get_world_state().await?;
        let mut message = String::from("**Opérateur, voici ce que j'ai pu récupérer sur la rotation hebdomadaire.\n\n**");

        message.push_str("__Teshin vend actuellement :__\n");
        let teshin = WarframeApi::get_teshin(&worldstate).await;
        match teshin {
            Some(teshin) => {
                message.push_str(&format!("{}.\n", teshin));
            }
            None => {
                message.push_str("Aucune information sur Teshin.\n");
            }
        }

        message.push_str("\n__Chasse à l'Archonte__\n");
        if let Some((boss, missions, blood_pact)) = WarframeApi::get_archon_hunt(&worldstate).await {
            message.push_str(&format!("L'Archonte actuel est : {}.\n", boss));
            message.push_str("Missions : ");
            for mission in &missions {
                message.push_str(&format!("{}, ", mission));
            }
            message.pop();
            message.pop();
            message.push_str(".\n");

            if blood_pact {
                message.push_str("Un pacte de sang passé dans des temps anciens doit être honoré. Kyrn et Héphaï, vous devrez réaliser cette chasse ensemble.\n");
            }
        } else {
            message.push_str("Aucune chasse à l'Archonte active pour le moment.\n");
        }

        message.push_str("\n__Circuit de Duviri__\n");
        match WarframeApi::get_circuit(&worldstate).await {
            Some(circuits) => {
                for (category, choices) in circuits {
                    if category == "hard" {
                        message.push_str("Steel Path : ");
                    }
                    else {
                        message.push_str("Normal : ");
                    }
                    for choice in &choices {
                        message.push_str(&format!("{}, ", choice));
                    }
                    message.pop();
                    message.pop();
                    message.push_str(".\n");
                }
            }
            None => message.push_str("Aucun circuit de Duviri disponible pour le moment.\n"),
        };

        message.push_str("\n__Missions avec Récompenses Orokin__\n");
        match WarframeApi::get_orokin_rewards(&worldstate).await {
            Some(missions) => {
                if !missions.is_empty() {
                    for mission in &missions {
                        message.push_str(&format!("- {} \n", mission));
                    }
                } else {
                    message.push_str("Aucune mission avec récompense Orokin détectée.\n");
                }
            }
            None => message.push_str("Impossible de récupérer les informations sur les récompenses Orokin.\n"),
        };

        Ok(message)
    }

    pub async fn embed_duviri() -> Result<CreateEmbed, Box<dyn Error + Send + Sync>> {
        let worldstate = WarframeApi::get_world_state().await?;

        let emotion = WarframeApi::get_duviri_emotions(&worldstate).await;
        let translated_emotion = Self::translate_emotion(&emotion);

        let colour = Colour::from_rgb(174, 194, 191);

        let mut embed = CreateEmbed::default();
        embed.author(|a| {
            a.name("Céphalon Poseidis")
             .icon_url("https://media.discordapp.net/attachments/1191550053014311086/1191550213865865246/LOGO_couleurs1.png?ex=67d6261c&is=67d4d49c&hm=d6a1750975a0173c9cc8da7cf362060d236b93966a067705cd5c1cd3f1d4a2b9&=&format=webp&quality=lossless&width=1683&height=856")
             .url("https://github.com/naguiagahnim/CephalonPoseidis")
        });
        embed.title("Duviri");
        embed.colour(colour);
        embed.description(&format!("L'émotion régissant actuellement Duviri est : {}\n", translated_emotion));
        if translated_emotion == "Joie" {
            embed.image("https://preview.redd.it/duviri-and-its-paragons-of-emotion-v0-nxu2j00nn1wa1.png?width=1080&crop=smart&auto=webp&s=ed0f760c4bced39bcf90e2ade68dfaa665981e21");
        }
        else if translated_emotion == "Colère" {
            embed.image("https://preview.redd.it/duviri-and-its-paragons-of-emotion-v0-feedzmson1wa1.png?width=1080&crop=smart&auto=webp&s=7f8a6d494a2e90454198295d78f9d924cccf52d8");
        }
        else if translated_emotion == "Chagrin" {
            embed.image("https://preview.redd.it/duviri-and-its-paragons-of-emotion-v0-6x9e8l4mn1wa1.png?width=1080&crop=smart&auto=webp&s=5f956fa3786a817f8ec83889b0f75e76bd63b8ca");
        }
        else if translated_emotion == "Peur" {
            embed.image("https://preview.redd.it/duviri-and-its-paragons-of-emotion-v0-ygvoww0on1wa1.png?width=1080&crop=smart&auto=webp&s=424379f75a0434dbc46e25421b0d41e01a0b3716");
        }
        else if translated_emotion == "Envie" {
            embed.image("https://preview.redd.it/duviri-and-its-paragons-of-emotion-v0-m7m6rvipn1wa1.png?width=1080&crop=smart&auto=webp&s=70b8ec709124f8a04494d98c383b431e5e60ee66");
        }
        else {
            embed.image("https://www.yaakadev.com/wp-content/themes/ryse/assets/images/no-image/No-Image-Found-400x264.png");
        }
    
        Ok(embed)
    }

    pub async fn embed_cetus() -> Result<CreateEmbed, Box<dyn Error + Send + Sync>> {
        let worldstate = WarframeApi::get_world_state().await?;

        let cetus = WarframeApi::get_cetus_cycle(&worldstate).await;
        let translated_cetus = Self::translate_cetus(&cetus);

        let colour = Colour::from_rgb(174, 194, 191);

        let mut embed = CreateEmbed::default();
        embed.title("Cetus");
        embed.author(|a| {
            a.name("Céphalon Poseidis")
             .icon_url("https://media.discordapp.net/attachments/1191550053014311086/1191550213865865246/LOGO_couleurs1.png?ex=67d6261c&is=67d4d49c&hm=d6a1750975a0173c9cc8da7cf362060d236b93966a067705cd5c1cd3f1d4a2b9&=&format=webp&quality=lossless&width=1683&height=856")
             .url("https://github.com/naguiagahnim/CephalonPoseidis")
        });
        embed.colour(colour);
        embed.description(&format!("Il fait {} sur Cetus.\n", translated_cetus));
        if translated_cetus == "jour" {
            embed.image("https://static.wikia.nocookie.net/warframe/images/6/69/Cetus.png/revision/latest/scale-to-width-down/1000?cb=20250112114907");
        }
        else if translated_cetus == "nuit" {
            embed.image("https://static.wikia.nocookie.net/warframe/images/4/4f/Brian-yu-cetus02.jpg/revision/latest/scale-to-width-down/1000?cb=20230430175700");
        }
        else {
            embed.image("https://www.yaakadev.com/wp-content/themes/ryse/assets/images/no-image/No-Image-Found-400x264.png");
        }
        Ok(embed)
    }

    pub async fn embed_necralisk() -> Result<CreateEmbed, Box<dyn Error + Send + Sync>> {
        let worldstate = WarframeApi::get_world_state().await?;

        let wyrm = WarframeApi::get_necralisk_bagarre(&worldstate).await;
        let translated_wyrm = Self::translate_necralisk(&wyrm);

        let colour = Colour::from_rgb(174, 194, 191);

        let mut embed = CreateEmbed::default();
        embed.title("Puy de Cambion");
        embed.author(|a| {
            a.name("Céphalon Poseidis")
             .icon_url("https://media.discordapp.net/attachments/1191550053014311086/1191550213865865246/LOGO_couleurs1.png?ex=67d6261c&is=67d4d49c&hm=d6a1750975a0173c9cc8da7cf362060d236b93966a067705cd5c1cd3f1d4a2b9&=&format=webp&quality=lossless&width=1683&height=856")
             .url("https://github.com/naguiagahnim/CephalonPoseidis")
        });
        embed.colour(colour);
        embed.description(&format!("Le Wyrm putrescent {} domine le Puy de Cambion, pour l'instant.\n", translated_wyrm));
        if translated_wyrm == "Vome" {
            embed.image("https://static.wikia.nocookie.net/warframe/images/1/11/VomeWyrm.png/revision/latest/scale-to-width-down/1000?cb=20230913133813&path-prefix=fr");
        }
        else if translated_wyrm == "Fass" {
            embed.image("https://static.wikia.nocookie.net/warframe/images/e/e1/FassWyrm.png/revision/latest/scale-to-width-down/1000?cb=20230913133745&path-prefix=fr");
        }
        else {
            embed.image("https://www.yaakadev.com/wp-content/themes/ryse/assets/images/no-image/No-Image-Found-400x264.png");
        }
        Ok(embed)
    }

    pub async fn embed_vallis() -> Result<CreateEmbed, Box<dyn Error + Send + Sync>> {
        let worldstate = WarframeApi::get_world_state().await?;

        let vallis = WarframeApi::get_orbvallis_temperature(&worldstate).await;
        let translated_vallis = Self::translate_vallis(&vallis);

        let colour = Colour::from_rgb(174, 194, 191);

        let mut embed = CreateEmbed::default();
        embed.title("Vallée Orbis");
        embed.author(|a| {
            a.name("Céphalon Poseidis")
             .icon_url("https://media.discordapp.net/attachments/1191550053014311086/1191550213865865246/LOGO_couleurs1.png?ex=67d6261c&is=67d4d49c&hm=d6a1750975a0173c9cc8da7cf362060d236b93966a067705cd5c1cd3f1d4a2b9&=&format=webp&quality=lossless&width=1683&height=856")
             .url("https://github.com/naguiagahnim/CephalonPoseidis")
        });
        embed.colour(colour);
        embed.description(&format!("Il fait {} dans la Vallée Orbis.\n", translated_vallis));
        embed.image("https://static.wikia.nocookie.net/warframe/images/4/4e/Vall%C3%A9e_Orbis.jpg/revision/latest/scale-to-width-down/1000?cb=20190107122459&path-prefix=fr");
        Ok(embed)
    }

    pub async fn embed_zariman() -> Result<CreateEmbed, Box<dyn Error + Send + Sync>> {
        let worldstate = WarframeApi::get_world_state().await?;

        let zariman = WarframeApi::get_zariman_rotation(&worldstate).await;
        let translated_zariman = Self::translate_zariman(&zariman);

        let colour = Colour::from_rgb(174, 194, 191);

        let mut embed = CreateEmbed::default();
        embed.title("Les Anges du Zariman");
        embed.author(|a| {
            a.name("Céphalon Poseidis")
             .icon_url("https://media.discordapp.net/attachments/1191550053014311086/1191550213865865246/LOGO_couleurs1.png?ex=67d6261c&is=67d4d49c&hm=d6a1750975a0173c9cc8da7cf362060d236b93966a067705cd5c1cd3f1d4a2b9&=&format=webp&quality=lossless&width=1683&height=856")
             .url("https://github.com/naguiagahnim/CephalonPoseidis")
        });
        embed.colour(colour);
        embed.description(&format!("Les {} occupent actuellement le Zariman.", translated_zariman));
        if translated_zariman == "Grineer" {
            embed.image("https://static.wikia.nocookie.net/warframe/images/7/71/GrineerLancier.jpg/revision/latest?cb=20130922222239&path-prefix=fr");
        }
        else if translated_zariman == "Corpus" {
            embed.image("https://steamuserimages-a.akamaihd.net/ugc/1827915226531836144/4C3680934666DAD177A2EE24FAD95DE6F1A84604/?imw=512&&ima=fit&impolicy=Letterbox&imcolor=%23000000&letterbox=false");
        }
        else {
            embed.image("https://www.yaakadev.com/wp-content/themes/ryse/assets/images/no-image/No-Image-Found-400x264.png");
        }
        
        Ok(embed)
    }
}