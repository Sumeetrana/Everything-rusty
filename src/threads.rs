use std::thread::spawn;

pub fn test_thread() {
    let mut x: u128 = 0u128;

    for i in 1..500_000_000 {
        x += i;
    }

    println!("{}", x);
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x: u128 = 0u128;

        for i in 1..500_000_000 {
            x += i;
        }

        println!("{}", x);
    };

    test_thread();

    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);

    handle.join();
    handle2.join();
}
