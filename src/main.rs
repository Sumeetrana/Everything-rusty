pub mod datetime;
pub mod mpsc_channels;
pub mod scope_threads;
pub mod serde;
pub mod threads;

fn main() {
    // datetime::test_stdtime();
    // datetime::test_chronos();
    // threads::test_threads();
    // threads::spawn_thread();
    // scope_threads::test_scoped_threads();
    // mpsc_channels::test_mpsc_channels();
    serde::test_serde();
}
