use rayon;

pub fn set_threading(num_threads: usize) {
    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
}