use std::env;

fn main() {
    dotenvy::from_path(".env").expect("Fichier .env introuvable pendant le build");

    let vars = ["DISCORD_TOKEN", "PP_ID", "CL_ID", "DI_ID"];
    for var in vars {
        let val = env::var(var).expect(&format!("Variable manquante : {}", var));
        println!("cargo:rustc-env={}={}", var, val);
    }
}
