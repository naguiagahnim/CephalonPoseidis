pub fn get_token() -> String {
    env!("DISCORD_TOKEN").to_string()
}

pub fn get_guild_pp_id() -> u64 {
    env!("PP_ID").parse().expect("Format de PP_ID foireux")
}

pub fn get_guild_cl_id() -> u64 {
    env!("CL_ID").parse().expect("Format de CL_ID foireux")
}

pub fn get_guild_di_id() -> u64 {
    env!("DI_ID").parse().expect("Format de DI_ID foireux")
}
