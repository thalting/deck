use compact_str::{format_compact, CompactString};
use std::{collections::HashMap, env, fs};

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Deck {
    pub commands: Vec<Card>,
}

#[derive(Debug, Deserialize)]
pub struct Card {
    pub name: CompactString,
    pub cmd: CompactString,
    pub args: Option<Vec<CompactString>>,
}

type HashDeck = HashMap<CompactString, (CompactString, Option<Vec<CompactString>>)>;

pub fn get() -> std::io::Result<HashDeck> {
    let config_path = format_compact!("{}/.config/deck/config.toml", env::var("HOME").unwrap());
    let file: CompactString = fs::read_to_string(&*config_path).unwrap().into();
    let config: Deck = toml::from_str(&file).unwrap();

    let mut map = HashMap::new();
    for card in config.commands {
        map.insert(card.name, (card.cmd, card.args));
    }

    Ok(map)
}
