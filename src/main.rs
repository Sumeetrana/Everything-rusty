pub mod datetime;
pub mod threads;

fn main() {
    // datetime::test_stdtime();
    // datetime::test_chronos();
    // threads::test_threads();
    threads::spawn_thread();
}
