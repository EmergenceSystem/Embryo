use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Embryo {
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbryoList {
    pub embryo_list: Vec<Embryo>,
}
