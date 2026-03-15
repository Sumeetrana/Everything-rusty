fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }

    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_string(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("blue"),
        String::from("green"),
    ];

    // print_elements(&colors);
    shorten_string(&mut colors);
    println!("{:#?}", colors);
}
