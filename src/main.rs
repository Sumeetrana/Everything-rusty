use std::fs;

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() {
    let text = fs::read_to_string("logs.txt");

    match text {
        Ok(value) => {
            let error_logs = extract_errors(value.as_str());
            println!("{:#?}", error_logs);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
