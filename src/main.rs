mod models;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use serde_xml_rs::{from_str, to_string};

use crate::models::Item;

fn main() {
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
