use super::api::WarframeApi;
use std::{collections::HashMap, error::Error};
use once_cell::sync::Lazy;

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

    pub async fn announce_cycles() -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut message = String::from("**Opérateur, voici les informations sur les cycles actuels.** \n\n");
        let worldstate = WarframeApi::get_world_state().await?;

        message.push_str("__Cycles actuels__ \n");
        let emotion = WarframeApi::get_duviri_emotions(&worldstate).await;
        let translated_emotion = Self::translate_emotion(&emotion);
        message.push_str(&format!("L'émotion régissant actuellement Duviri est : {}\n", translated_emotion));

        let cetus = WarframeApi::get_cetus_cycle(&worldstate).await;
        let translated_cetus = Self::translate_cetus(&cetus);
        message.push_str(&format!("Il fait {} sur Cetus.\n", translated_cetus));

        let necralisk = WarframeApi::get_necralisk_bagarre(&worldstate).await;
        let translated_necralisk = Self::translate_necralisk(&necralisk);
        message.push_str(&format!("Le Wyrm putrescent {} domine le Puy de Cambion, pour l'instant.\n", translated_necralisk));

        let orbvallis = WarframeApi::get_orbvallis_temperature(&worldstate).await;
        let translated_vallis = Self::translate_vallis(&orbvallis);
        message.push_str(&format!("Il fait {} dans la Vallée Orbis.\n", translated_vallis));

        let zariman = WarframeApi::get_zariman_rotation(&worldstate).await;
        let translated_zariman = Self::translate_zariman(&zariman);
        message.push_str(&format!("La faction occupant actuellement le Zariman est la faction {}.", translated_zariman));

        Ok(message)
    }

    pub async fn announce_weekly_reset() -> Result<String, Box<dyn Error + Send + Sync>> {
        let worldstate = WarframeApi::get_world_state().await?;
        let mut message = String::from("**Opérateur, voici ce que j'ai pu récupérer sur la rotation hebdomadaire.\n\n**");

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
}