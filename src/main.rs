use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    // let text = fs::read_to_string("logs.txt");

    // match text {
    //     Ok(value) => {
    //         let error_logs = extract_errors(value.as_str());
    //         println!("{:#?}", error_logs);
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(()) => println!("Error logs success"),
    //             Err(error) => println!("{:#?}", error),
    //         }
    //     }
    //     Err(error) => {
    //         println!("{}", error);
    //     }
    // }

    let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");

    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n")).expect("failed to write errors.txt");
}
