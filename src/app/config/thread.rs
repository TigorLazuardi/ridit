use rayon::{ThreadPoolBuildError, ThreadPoolBuilder};

pub fn configure_concurrency(threads: usize) -> Result<(), ThreadPoolBuildError> {
    ThreadPoolBuilder::new().num_threads(threads).build_global()
}
