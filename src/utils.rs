use cached::{cached_key, SizedCache};
use ndarray::{Array, IxDyn};
use nifti::{InMemNiftiObject, IntoNdArray, NiftiHeader, NiftiObject};
use rayon;

pub fn set_threading(num_threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
}

cached_key! {
    READ: SizedCache<String, Array<f64, IxDyn>> = SizedCache::with_size(10);
    Key = { format!("{}", path)};
    fn read_nd_image(path: String) -> Array<f64, IxDyn> = {
        let nifti_object = InMemNiftiObject::from_file(path).expect("Nifti file is unreadable.");
        let volume = nifti_object.into_volume();
        volume.into_ndarray::<f64>().unwrap()
    }
}

pub fn read_header(path: &str) -> NiftiHeader {
    let nifti_object = InMemNiftiObject::from_file(path).expect("Nifti file in unreadable.");
    nifti_object.header().clone()
}
