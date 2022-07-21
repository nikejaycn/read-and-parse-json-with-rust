use std::fs::File;
use std::io::{ Read, Result, Error };
use std::path::Path;
use serde::{ Deserialize, Serialize };
use serde_json::Value as JsonValue;
use std::collections::HashMap;
// base title json: https://tinfoil.media/repo/db/titles.json

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// Option Aall is used to represent null values in the JSON
struct Game {
    pub banner_url: Option<JsonValue>,
    pub category: Option<Vec<JsonValue>>,
    pub description: Option<JsonValue>,
    pub developer: Option<JsonValue>,
    pub id: Option<JsonValue>,
    pub is_demo: Option<JsonValue>,
    pub language: Option<JsonValue>,
    pub name: Option<JsonValue>,
    pub nsu_id: Option<usize>,
    pub number_of_players: Option<usize>,
    pub publisher: Option<JsonValue>,
    pub rank: Option<JsonValue>,
    pub rating_content: Option<Vec<JsonValue>>,
    pub size: Option<JsonValue>,
    pub version: Option<JsonValue>,
}

fn load_demo_json() -> Result<HashMap<String, Game>> {
    let demo_file_path = Path::new("appdata/demo.json");
    let file = File::open(demo_file_path)?;
    // get file metadata
    let metadata = file.metadata()?;
    println!("The File Size is: {:?} Bytes", metadata.len());

    let content: HashMap<String, Game> = match serde_json::from_reader(&file) {
        Ok(content) => {
            println!("file loaed success");
            content
        },
        Err(e) => {
            println!("file loaed error: {}", e);
            Err(e)?
        }
    };

    println!("{:?}", content);
    println!("----------- end load -----------");

    Ok(content)
}

fn main() {
    let content = load_demo_json();
    // println content key 0100000000010000 name
    println!("{:?}", content.unwrap().get("0100000000010000").unwrap().name);
    
}
