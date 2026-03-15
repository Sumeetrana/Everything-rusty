fn print_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];

    print_elements(&colors);
}
