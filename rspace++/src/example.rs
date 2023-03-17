use serde::{Deserialize, Serialize};

pub struct Channel {
    pub name: String,
}

pub struct Name {
    pub first: String,
    pub last: String,
}

pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

pub struct Entry {
    pub name: Name,
    pub address: Address,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
