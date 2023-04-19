use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct Name {
    pub first: String,
    pub last: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct Entry {
    pub name: Name,
    pub address: Address,
    pub email: String,
    pub phone: String,
    pub pos: u8,
    pub posStr: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash)]
pub struct Printer;

impl Printer {
    pub fn print_entry(&self, entry: &Entry) -> () {
        let name_str = format!("{}, {}", entry.name.last, entry.name.first);
        let addr_str = format!(
            "{}, {}, {}, {}",
            entry.address.street, entry.address.city, entry.address.state, entry.address.zip
        );

        println!(
            r#"
=== ENTRY ===
name:    {}
address: {}
email:   {}
phone:   {}
"#,
            name_str, addr_str, entry.email, entry.phone
        );
    }
}
