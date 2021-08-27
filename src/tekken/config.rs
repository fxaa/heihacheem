use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs::read, path::Path};

#[derive(Debug, Deserialize)]
struct MokujinGlossaryEntry {
    online_webpage: String,
    portrait: String,
    proper_name: String,
    name: String,
    local_json: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BlockFrame {
    Frame(String),
    Unblockable(i32),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Aliases {
    Multiple(Vec<String>),
    Single(String),
}

#[derive(Debug, Deserialize)]
pub struct MokujinMoveEntry {
    #[serde(rename = "Alias")]
    pub alias: Option<Aliases>,
    #[serde(rename = "Block frame")]
    pub block_frame: BlockFrame,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Counter hit frame")]
    pub counter_hit_frame: String,
    #[serde(rename = "Damage")]
    pub damage: String,
    #[serde(rename = "Gif")]
    pub gif: Option<String>,
    #[serde(rename = "Hit frame")]
    pub hit_frame: String,
    #[serde(rename = "Hit level")]
    pub hit_level: String,
    #[serde(rename = "Notes")]
    pub notes: String,
    #[serde(rename = "Start up frame")]
    pub start_up_frame: String,
}

#[derive(Debug)]
struct MokujinCharacter {
    name: String,
    moves: Vec<MokujinMoveEntry>,
}

pub fn load_mokujin_config() -> Result<HashMap<String, Vec<MokujinMoveEntry>>, Box<dyn Error>> {
    // TODO use env var to determine the path
    let json_dir = Path::new("../mokujin/json");
    let glossary_config = Path::new("../mokujin/src/resources/character_misc.json");
    let glossary_file_str = String::from_utf8(read(glossary_config)?)?;
    let glossary_entries: Vec<MokujinGlossaryEntry> =
        serde_json::from_str(glossary_file_str.as_str()).expect("failed to parse json");
    Ok(glossary_entries
        .iter()
        .map(|entry| {
            (
                entry.proper_name.to_owned(),
                read(String::from(json_dir.to_str().unwrap()) + "/" + entry.local_json.as_str())
                    .expect(&("failed to parse".to_owned() + entry.proper_name.as_str())),
            )
        })
        .map(|(name, bytes)| {
            (
                name,
                String::from_utf8(bytes).expect("failed to parse utf8"),
            )
        })
        .map(|(name, json)| {
            (
                name.to_owned(),
                serde_json::from_str::<Vec<MokujinMoveEntry>>(json.as_str())
                    .expect(&("failed to parse ".to_owned() + name.clone().as_str())),
            )
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, Vec<MokujinMoveEntry>>, (name, moves)| {
                acc.insert(name, moves);
                acc
            },
        ))
}
