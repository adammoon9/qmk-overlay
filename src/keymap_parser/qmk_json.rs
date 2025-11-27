use deser_hjson;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io, panic};
use thiserror::Error;

// #[derive(Serialize, Deserialize)]
// struct Keymap {
//     layers: Vec<Vec<String>>,
// }
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] io::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Failed to parse HJSON: {0}")]
    HjsonError(#[from] deser_hjson::Error),

    #[error("Keymap file not found at: {0}")]
    KeymapNotFound(PathBuf),
}

#[derive(Serialize, Deserialize, Debug)]
struct Keymap {
    keyboard: String,
    keymap: String,
    layout: String,
    layers: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeycodeFile {
    keycodes: HashMap<String, KeycodeInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeycodeInfo {
    key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<Vec<String>>,
}

pub fn parse_keymap(keymap_path: &Path) -> Result<Keymap, ParserError> {
    // let keymap_dir =
    //     Path::new("/home/apm/qmk_firmware/keyboards/ferris/keymaps/apmferris/keymap.json");
    if !Path::exists(&keymap_path) {
        return Err(ParserError::KeymapNotFound(keymap_path.to_path_buf()));
    }

    let contents = fs::read_to_string(keymap_path)?;
    let keymap = serde_json::from_str::<Keymap>(&contents)?;

    Ok(keymap)
}

pub fn parse_keycodes(keycodes_dir: &Path) -> HashMap<String, KeycodeInfo> {
    // let keycode_directory = Path::new("/home/apm/qmk_firmware/data/constants/keycodes");
    let keycode_files = fs::read_dir(keycodes_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("hjson"));

    let mut keycodes_map = HashMap::new();

    for entry in keycode_files {
        let path = entry.path();
        println!("Reading keycode file: {}", &path.display());
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Unable to read keycode file: {}", path.display()));

        match deser_hjson::from_str::<KeycodeFile>(&contents) {
            Ok(keycode_file) => {
                let keycodes_count = keycode_file.keycodes.len();
                keycodes_map.extend(keycode_file.keycodes);
                println!(
                    "Loaded {} keycodes from {}",
                    keycodes_count,
                    path.file_name().unwrap().to_str().unwrap()
                );
            }
            Err(e) => {
                println!("Skipping file {} - parse error: {}", path.display(), e);
                continue;
            }
        }
    }

    println!("\nTotal keycodes loaded: {}", keycodes_map.len());
    keycodes_map
}
