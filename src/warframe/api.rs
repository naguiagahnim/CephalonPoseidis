use reqwest;
use serde_json::Value;

pub struct WarframeApi;

impl WarframeApi {

    pub async fn get_world_state() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr";
        let body = reqwest::get(url).await?.json().await?;
        Ok(body)
    }

    pub async fn get_duviri_emotions(world_state : &Value) -> Option<String> {
        world_state["duviriCycle"]["state"].as_str().map(String::from)
    }

    pub async fn get_cetus_cycle(world_state : &Value) -> Option<String> {
        world_state["cetusCycle"]["state"].as_str().map(String::from)
    }

    pub async fn get_necralisk_bagarre(world_state : &Value) -> Option<String> {
        world_state["cambionCycle"]["state"].as_str().map(String::from)
    }

    pub async fn get_orbvallis_temperature(world_state : &Value) -> Option<String> {
        world_state["vallisCycle"]["state"].as_str().map(String::from)
    }

    pub async fn get_zariman_rotation(world_state : &Value) -> Option<String> {
        world_state["zarimanCycle"]["state"].as_str().map(String::from)
    }

    pub async fn get_deep_archimedia(world_state : &Value) -> Option<String> {
        let _ = world_state;
        todo!("quand l'api remarchera")
    }

    pub async fn get_archon_hunt(world_state: &Value) -> Option<(String, Vec<String>, bool)> {
        let archon_hunt = world_state.get("archonHunt")?;
        
        let boss = archon_hunt.get("boss")?.as_str()?.to_string();
        
        let missions = archon_hunt.get("missions")?
            .as_array()?
            .iter()
            .filter_map(|mission| mission.get("type")?.as_str().map(String::from))
            .collect::<Vec<String>>();

        let blood_pact = missions.iter().any(|mission_type| 
            mission_type == "Interception" || mission_type == "Espionnage"
        );

        if missions.is_empty() {
            None
        } else {
            Some((boss, missions, blood_pact))
        }
    }

    pub async fn get_circuit(world_state: &Value) -> Option<Vec<(String, Vec<String>)>> {
        world_state.get("duviriCycle")?
            .get("choices")?
            .as_array()?
            .iter()
            .filter_map(|category| {
                let category_name = category.get("category")?.as_str()?;
                let choices = category.get("choices")?
                    .as_array()?
                    .iter()
                    .filter_map(|choice| choice.as_str().map(String::from))
                    .collect::<Vec<String>>();
                
                if !choices.is_empty() {
                    Some((category_name.to_string(), choices))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, Vec<String>)>>()
            .into_iter()
            .collect::<Vec<_>>()
            .into()
    }

    pub async fn get_orokin_rewards(world_state: &Value) -> Option<Vec<String>> {
        let invasions = world_state.get("invasions")?.as_array()?;
    
        let missions_with_orokin = invasions
            .iter()
            .filter_map(|invasion| {
                let attacker = invasion.get("attacker")?;
                let defender = invasion.get("defender")?;
                let attacker_reward = attacker.get("reward")?;
                let defender_reward = defender.get("reward")?;
    
                let attacker_items = attacker_reward.get("itemString").and_then(|v| v.as_str()).unwrap_or("");
                let defender_items = defender_reward.get("itemString").and_then(|v| v.as_str()).unwrap_or("");
    
                let has_orokin = attacker_items.contains("Catalyseur Orokin") || attacker_items.contains("Réacteur Orokin")
                    || defender_items.contains("Catalyseur Orokin") || defender_items.contains("Réacteur Orokin");
    
                if has_orokin {
                    let node = invasion.get("node").and_then(|v| v.as_str()).unwrap_or("Lieu inconnu");
                    let desc = invasion.get("desc").and_then(|v| v.as_str()).unwrap_or("Invasion inconnue");
                    Some(format!("{} - {}", node, desc))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
    
        if missions_with_orokin.is_empty() {
            None
        } else {
            Some(missions_with_orokin)
        }
    }
    

    pub async fn get_teshin(world_state: &Value) -> Option<String> {
        world_state["steelPath"]["currentReward"]["name"].as_str().map(String::from)
    }
}