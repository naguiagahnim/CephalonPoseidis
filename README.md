<p align="center">
    <h1 align="center">Céphalon Poseidis</h1>
</p>
<p align="center">
  <img src="https://media.discordapp.net/attachments/1191550053014311086/1191550151391727708/LOGO_ZEBI.png?ex=67f86d0e&is=67f71b8e&hm=c2255dd56ee54eb3f35ffdb9fe93b30e1c6e812bd8582da99d7d08731af8768f&=&format=webp&quality=lossless&width=160&height=160" alt="Clan Logo">
</p>
<p align="center">
    <em><code>❯ Cycles, Weekly Reset and more to come</code></em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/last-commit/naguiagahnim/CephalonPoseidis?style=flat&logo=git&logoColor=white&color=ff00bc" alt="last-commit">
	<img src="https://img.shields.io/github/languages/top/naguiagahnim/CephalonPoseidis?style=flat&color=ff00bc" alt="repo-top-language">
</p>

<p align="center">
    <em>Built with:</em>
</p>
<p align="center">
	<img src="https://img.shields.io/badge/Rust-%23000000.svg?style=flat&logo=rust&logoColor=white" alt="Rust">
</p>

<br>

## About

Céphalon Poseidis is a Discord bot developed in Rust, designed to provide information about game cycles, weekly resets, and more features to come, specifically for our Warframe clan, The Ashen Tempestariis.

## Prerequisites

To run the bot, you will need:

- Rust (installable via [rustup](https://rustup.rs/))
- A `.env` file containing the necessary configurations

### `.env` Configuration

The `.env` file must contain the variables you need, considering you updated the source code (especially `config.rs` and the rest of the code using it).

## Running the Bot

1. Clone the repository:
    ```
    git clone https://github.com/naguiagahnim/CephalonPoseidis.git
    cd CephalonPoseidis
    ```
2. Ensure you have configured the `.env` file.

3. Run the bot:
    ```
    cargo run
    ```

    Or, if you want an executable:

    ```
    cargo build --release
    ./target/release/CephalonPoseidis
    ```

## Important Notes

This bot is specifically designed for my needs and has limited modularity. Any modifications will require direct adaptation of the source code, especially the parts using the channel and guild ids.

## Credits

Clan logo by Themaninthewall678.
