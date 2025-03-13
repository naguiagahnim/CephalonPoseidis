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
        let events = world_state.get("events")?.as_array()?;

        let missions_with_orokin = events
            .iter()
            .filter_map(|event| {
                let event_description = event.get("description").and_then(|v| v.as_str())?;
                let rewards = event.get("rewards")?.as_array()?;

                let has_orokin_reward = rewards.iter().any(|reward| {
                    let item_string = reward.get("itemString").and_then(|v| v.as_str()).unwrap_or_default();
                    item_string.contains("Catalyst") || item_string.contains("Reactor")
                });

                if has_orokin_reward {
                    Some(event_description.to_string())
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
}