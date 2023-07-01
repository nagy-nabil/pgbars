use serde::{Deserialize, Serialize};
use serde_json;

// To get started with serde_json, you must first implement the Serialize and Deserialize traits on your types.
#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    name: String,
    age: usize,
    verified: bool,
}

pub fn read() -> Result<(), serde_json::Error> {
    let json = r#"
    {
        "name": "nagy",
        "age": 21,
        "verified": true
    }
    "#;
    // parse the string to json
    // type could be any type that implement deserialize
    let person: Person = serde_json::from_str(json)?;
    println!(
        "🪵 [json.rs:21]~ token ~ \x1b[0;32mperson\x1b[0m = {:?}",
        person
    );
    return Ok(());
}
