use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
struct Dog {
    name: String,
    year_born: i32,
}

pub fn test_serde() {
    let dog01 = Dog {
        name: "Cheyenne".to_string(),
        year_born: 2021,
    };

    match to_string(&dog01) {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{:?}", error),
    }
}
