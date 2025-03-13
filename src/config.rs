use std::env;

use dotenv::dotenv;

pub async fn get_token() -> String {
    dotenv().ok();
    match env::var("DISCORD_TOKEN") {
        Ok(val) => val,
        Err(e) => panic!("Erreur cl  token: {}", e),
    }
}

pub async fn get_clan_cid() -> u64 {
    dotenv().ok();
    match env::var("CLAN_ID") {
        Ok(val) => val.parse().expect("Invalid CLAN_ID format"),
        Err(e) => panic!("Erreur cl channel clan: {}", e),
    }
}

pub async fn get_guild_id() -> u64 {
    dotenv().ok();
    match env::var("GUILD_ID") {
        Ok(val ) => val.parse().expect("Invalid GUILD_ID format"),
        Err(e) => panic!("Erreur cl guild: {}", e),
    }
}