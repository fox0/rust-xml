#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(rename = "Item", default)]
    pub items: Vec<Item>,
}
