// External USEs
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Character {
    pub name: String,
    pub data: Value,
    pub specie: Option<String>,
    pub class: Option<String>,
    pub universe: Option<String>
}

#[derive(Serialize)]
pub struct Weapon {
    pub name: String,
    pub data: Value,
    pub universe: String,
    pub owner: Option<String>
}
