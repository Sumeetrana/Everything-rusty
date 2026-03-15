fn print_elements(elements: &Vec<String>) {
    // for element in elements {
    //     println!("{}", element);
    // }

    elements.iter().for_each(|el| println!("{}", el));
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];

    print_elements(&colors);
}
