use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::env;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Embryo {
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbryoList {
    pub embryo_list: Vec<Embryo>,
}

const CONF_FILE : &str = "emergence.conf";

pub fn read_emergence_conf() -> Option<HashMap<String, HashMap<String, String>>> {
    let config_path = match dirs::config_dir() {
        Some(mut path) => {
            path.push("emergence");
            path.push(CONF_FILE);
            path
        }
        None => return None, 
    };

    if !config_path.exists() {
        // Try %APPDATA% on Windows
        let mut appdata_path = PathBuf::new();
        if let Some(appdata) = std::env::var_os("APPDATA") {
            appdata_path.push(appdata);
            appdata_path.push("emergence");
            appdata_path.push(CONF_FILE);

            if appdata_path.exists() {
                return Some(read_file(&appdata_path));
            }
        }

        return None;
    }

    Some(read_file(&config_path))
}

pub fn read_file(path: &Path) -> HashMap<String, HashMap<String, String>> {
    let file = File::open(path).expect(&format!("Can't open path : {:?}", path));
    let reader = io::BufReader::new(file);
    let mut map = HashMap::new();
    let mut current_section = String::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                map.insert(current_section.clone(), HashMap::new());
            } else {
                let parts: Vec<&str> = line.splitn(2, '=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let value = parts[1].trim_matches('"').to_string();
                    if let Some(section_map) = map.get_mut(&current_section) {
                        section_map.insert(parts[0].to_string(), value);
                    }
                }
            }
        }
    }

    map
}

pub fn get_em_disco_url() -> String {
    match env::var("server_url") {
        Ok(url) => url,
        Err(_) => {
            let config_map = read_emergence_conf().unwrap_or_default();
            match config_map.get("em_disco").and_then(|em_disco| em_disco.get("server_url")) {
                Some(url) => url.clone(),
                None => "http://localhost:8080".to_string(),
            }
        },
    }
}

pub fn merge_lists_by_url(mut uri_list: Vec<Embryo>, other_list: Vec<Embryo>) -> Vec<Embryo> {
    uri_list.extend(other_list);

    let mut added_elements = HashSet::new();

    let result: Vec<_> = uri_list
        .into_iter()
        .filter(|embryo| {
            let has_duplicate_url = embryo.properties.iter().any(|(name, value)| {
                name == "url" && !added_elements.insert(value.clone())
            });

            !has_duplicate_url
        })
    .collect();

    result
}
