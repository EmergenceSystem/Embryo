use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Embryo {
    properties: Vec<EmPair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmPair {
    name: String,
    value: String,
}
