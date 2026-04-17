use std::thread::spawn;

pub fn test_thread() {
    let mut x: u128 = 0u128;

    for i in 1..500 {
        x += i;
    }

    println!("Main thread is finished...let's go and check the worker threads");
}

pub fn spawn_thread() {
    let thread_fn = || {
        let mut x: u128 = 0u128;

        for i in 1..500_000_000 {
            x += i;
        }

        println!("{}", x);
    };

    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);

    loop {
        test_thread();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are done, let's go get out of here");
            break;
        }
    }
}
