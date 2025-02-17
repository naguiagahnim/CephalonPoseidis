use reqwest;
use serde_json::Value;

pub struct WarframeApi;

impl WarframeApi {
    pub async fn get_world_state() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc";
        let response = reqwest::get(url).await?;
        response.json().await
    }
}