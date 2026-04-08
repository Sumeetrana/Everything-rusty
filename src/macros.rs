macro_rules! say_hello {
    () => {
        println!("Hello world");
    };
}

macro_rules! repeat_message {
    ($msg: expr, $times: expr) => {
        for _ in 0..$times {
            println!("{}", $msg);
        }
    };
}

macro_rules! create_vector {
    ($type: ty) => {
        fn new_vector() -> Vec<$type> {
            Vec::new()
        }
    };
}

create_vector!(i32);

fn main() {
    say_hello!();

    repeat_message!("Rust is awesome!", 3);

    let my_vec = new_vector();
    println!("Create a vector of type i32: {:?}", my_vec);
}
