use load_dotenv::load_dotenv;

pub async fn get_token() -> String {
    load_dotenv!();
    let token = env!("DISCORD_TOKEN");
    String::from(token)
}

pub async fn get_clan_cid() -> u64 {
    load_dotenv!();
    let cid = env!("CLAN_ID");
    let new_cid = cid.parse().unwrap();
    new_cid
}