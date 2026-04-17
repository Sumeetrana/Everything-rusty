pub fn test_scoped_threads() {
    let age = 34;

    let print_age = move || {
        println!("Your age is {age}");
    };

    let _result = std::thread::spawn(print_age).join();

    println!("Finished printing age");
}
