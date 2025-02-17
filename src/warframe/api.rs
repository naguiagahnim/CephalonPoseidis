use reqwest;
use serde_json::Value;

pub struct WarframeApi;

impl WarframeApi {

    pub async fn get_duviri_emotions() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("à faire")
    }

    pub async fn get_cetus_cycle() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("same")
    }

    pub async fn get_necralisk_bagarre() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("mais qui va gagner ?")
    }

    pub async fn get_orbvallis_temperature() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("le pire monde ouvert du jeu")
    }

    pub async fn get_zariman_rotation() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("Bizarre les types")
    }

    pub async fn get_deep_archimedia() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("a")
    }

    pub async fn get_teshin_formas() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("a")
    }

    pub async fn get_archon_hunt() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("a")
    }
    
    pub async fn get_circuit() -> Result<Value, reqwest::Error> {
        let url = "https://api.warframestat.us/pc/fr/...";
        let response = reqwest::get(url).await?;
        //TODO response.json().await
        todo!("a")
    }
}