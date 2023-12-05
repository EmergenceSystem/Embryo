use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Embryo {
    pub properties: Vec<EmPair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmPair {
    pub name: String,
    pub value: String,
}
