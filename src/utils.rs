use ndarray::{Array, IxDyn};
use nifti::{InMemNiftiObject, IntoNdArray, NiftiHeader, NiftiObject};
use rayon;

pub fn set_threading(num_threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
}

pub fn extract_volume(nifti_object: InMemNiftiObject) -> Array<f64, IxDyn> {
    let volume = nifti_object.into_volume();
    volume.into_ndarray::<f64>().unwrap()
}

pub fn read_nifti(path: &String) -> InMemNiftiObject {
    InMemNiftiObject::from_file(path).expect("Nifti file is unreadable.")
}

pub fn read_header(path: &String) -> NiftiHeader {
    NiftiHeader::from_file(path).unwrap()
}
