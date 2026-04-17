pub mod datetime;
pub mod scope_threads;
pub mod threads;
pub mod mpsc_channels;

fn main() {
    // datetime::test_stdtime();
    // datetime::test_chronos();
    // threads::test_threads();
    // threads::spawn_thread();
    // scope_threads::test_scoped_threads();
    mpsc_channels::test_mpsc_channels();
}
