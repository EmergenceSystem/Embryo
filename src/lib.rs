use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Embryo {
    properties: Vec<NameValuePair>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NameValuePair {
    name: String,
    value: String,
}
