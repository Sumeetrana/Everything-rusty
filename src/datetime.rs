use std::{
    ops::Sub,
    time::{Duration, Instant},
};

pub fn test_stdtime() {
    let dur1 = Duration::from_secs(15);
    println!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(15500);

    // let dur3 = dur1.sub(dur2);
    let dur3 = dur1.checked_sub(dur2);

    println!("{:?}", dur3.unwrap_or_default());
}
