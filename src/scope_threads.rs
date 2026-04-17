struct Person {
    first_name: String,
}

pub fn test_scoped_threads() {
    let age = 34;
    let person01 = Person {
        first_name: String::from("Trevor"),
    };

    let print_age = move || {
        println!("Your age is {age}");
        println!("your name is {}", person01.first_name);
    };

    println!("Your age is {age}");
    println!("your name is {}", person01.first_name); // Error

    let _result = std::thread::spawn(print_age).join();

    println!("Finished printing age");
}
