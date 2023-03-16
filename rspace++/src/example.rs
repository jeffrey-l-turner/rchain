// use std::error::Error;

pub struct Channel {
    name: String,
}

struct Name {
    first: String,
    last: String,
}

struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

pub struct Entry {
    name: Name,
    address: Address,
    email: String,
    phone: String,
}

fn print_entry(entry: &Entry) {
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

fn main() {
    let alice = Entry {
        name: Name {
            first: "Alice".to_string(),
            last: "Lincoln".to_string(),
        },
        address: Address {
            street: "777 Ford St.".to_string(),
            city: "Crystal Lake".to_string(),
            state: "Idaho".to_string(),
            zip: "223322".to_string(),
        },
        email: "alicel@ringworld.net".to_string(),
        phone: "787-555-1212".to_string(),
    };

    print_entry(&alice);
}
