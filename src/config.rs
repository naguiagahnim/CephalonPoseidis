use std::env;

use dotenv::dotenv;

pub async fn get_token() -> String {
    dotenv().ok();
    match env::var("DISCORD_TOKEN") {
        Ok(val) => val,
        Err(e) => panic!("Erreur cl  token: {}", e),
    }
}

pub async fn get_guild_pp_id() -> u64 {
    dotenv().ok();
    match env::var("PP_ID") {
        Ok(val ) => val.parse().expect("Invalid GUILD_ID format"),
        Err(e) => panic!("Erreur cl guild: {}", e),
    }
}

pub async fn get_guild_cl_id() -> u64 {
    dotenv().ok();
    match env::var("CL_ID") {
        Ok(val ) => val.parse().expect("Invalid GUILD_ID format"),
        Err(e) => panic!("Erreur cl guild: {}", e),
    }
}

pub async fn get_guild_di_id() -> u64 {
    dotenv().ok();
    match env::var("DI_ID") {
        Ok(val ) => val.parse().expect("Invalid GUILD_ID format"),
        Err(e) => panic!("Erreur cl guild: {}", e),
    }
}