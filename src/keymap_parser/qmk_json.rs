use deser_hjson;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};
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
pub struct Keymap {
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

pub fn load_keymap(keymap_path: &Path) -> Result<Keymap, ParserError> {
    // let keymap_dir =
    //     Path::new("/home/apm/qmk_firmware/keyboards/ferris/keymaps/apmferris/keymap.json");
    if !Path::exists(&keymap_path) {
        return Err(ParserError::KeymapNotFound(keymap_path.to_path_buf()));
    }

    let contents = fs::read_to_string(keymap_path)?;
    let keymap = serde_json::from_str::<Keymap>(&contents)?;

    Ok(keymap)
}

pub fn parse_keymap_layers(
    keymap: &Keymap,
    keycodes: &HashMap<String, KeycodeInfo>,
) -> Vec<Vec<String>> {
    let mut parsed_keymap: Vec<Vec<String>> =
        vec![vec![String::new(); keymap.layers[0].len()]; keymap.layers.len()];

    for (i, layer) in keymap.layers.iter().enumerate() {
        for (j, key) in layer.iter().enumerate() {
            println!("{} : {}", key, get_keycode_label(key, keycodes));
            parsed_keymap[i][j] = get_keycode_label(key, keycodes);
        }
    }

    parsed_keymap
}

pub fn parse_keycodes(keycodes_dir: &Path) -> Result<HashMap<String, KeycodeInfo>, ParserError> {
    let entries = fs::read_dir(keycodes_dir)?;
    let mut keycodes_map = HashMap::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) != Some("hjson") {
            continue;
        }

        println!("Reading keycode file: {}", &path.display());
        let contents = fs::read_to_string(&path)?;

        match deser_hjson::from_str::<KeycodeFile>(&contents) {
            Ok(keycode_file) => {
                let keycodes_count = keycode_file.keycodes.len();
                // keycodes_map.extend(keycode_file.keycodes);
                for (_, info) in keycode_file.keycodes {
                    keycodes_map.insert(info.key.clone(), info.clone());

                    if let Some(aliases) = &info.aliases {
                        for alias in aliases {
                            if !alias.trim().is_empty() {
                                keycodes_map.insert(alias.clone(), info.clone());
                            }
                        }
                    }
                }

                println!(
                    "Loaded {} keycodes from {}",
                    keycodes_count,
                    path.file_name().unwrap().to_str().unwrap()
                );
            }
            Err(e) => {
                println!("Warning: Skipping {} - parse error: {}", path.display(), e);
                continue;
            }
        }
    }
    let extra_keycodes_path = keycodes_dir.join("extras/keycodes_us_international_0.0.1.hjson");
    if extra_keycodes_path.exists() {
        let extra_keycodes = fs::read_to_string(&extra_keycodes_path)?;
        match deser_hjson::from_str::<KeycodeFile>(&extra_keycodes) {
            Ok(keycode_file) => {
                let keycodes_count = keycode_file.keycodes.len();
                // keycodes_map.extend(keycode_file.keycodes);
                for (_, info) in keycode_file.keycodes {
                    keycodes_map.insert(info.key.clone(), info.clone());
                    if let Some(aliases) = &info.aliases {
                        for alias in aliases {
                            if !alias.trim().is_empty() {
                                keycodes_map.insert(alias.clone(), info.clone());
                            }
                        }
                    }
                }
                println!(
                    "Loaded {} keycodes from {}",
                    keycodes_count,
                    extra_keycodes_path.file_name().unwrap().to_str().unwrap()
                );
            }
            Err(e) => {
                println!(
                    "Warning: Skipping {} - parse error: {}",
                    extra_keycodes_path.display(),
                    e
                );
            }
        }
    }

    println!("\nTotal keycodes loaded: {}", keycodes_map.len());
    Ok(keycodes_map)
}

pub fn get_keycode_label(keycode: &str, keycodes: &HashMap<String, KeycodeInfo>) -> String {
    keycodes
        .get(keycode)
        .and_then(|info| {
            info.label
                .as_ref()
                .filter(|l| !l.trim().is_empty())
                .cloned()
        })
        .unwrap_or_else(|| keycode.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keycode_lookup() {
        let mut keycodes = HashMap::new();

        keycodes.insert(
            "KC_A".to_string(),
            KeycodeInfo {
                key: "KC_A".to_string(),
                group: Some("basic".to_string()),
                label: Some("A".to_string()),
                aliases: Some(vec!["A".to_string()]),
            },
        );

        assert_eq!(get_keycode_label("KC_A", &keycodes), "A");
        assert_eq!(get_keycode_label("A", &keycodes), "A");
        assert_eq!(get_keycode_label("UNKNOWN", &keycodes), "UNKNOWN");
    }
}
