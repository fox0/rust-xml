mod models;
#[cfg(test)]
mod tests;

extern crate serde;
extern crate serde_xml_rs;

use validator::Validate;
use serde_xml_rs::{from_str, to_string};

use crate::models::{AuthData, Item};

fn main() {
    let a = AuthData {
        login: "fox".to_string(),
        pass: "Trixie is Best Pony".to_string(),
        institution_id: None,
    };
    a.validate().unwrap();

    let xml = to_string(&a).unwrap();
    dbg!(xml);

    let src = r#"<Item><name>Banana</name><source>Store</source></Item>"#;
    // let should_be = Item {
    //     name: "Banana".to_string(),
    //     source: "Store".to_string(),
    // };

    let item: Item = from_str(src).unwrap();
    dbg!(&item);
    // assert_eq!(item, should_be);

    let reserialized_item = to_string(&item).unwrap();
    dbg!(&reserialized_item);
    assert_eq!(src, reserialized_item);
}
