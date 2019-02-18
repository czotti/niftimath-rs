use ndarray::{Array3, Ix3};
use nifti::{DataElement, InMemNiftiObject, IntoNdArray, NiftiHeader, NiftiObject};
use num_traits::AsPrimitive;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum Elem {
    Image(Array3<f64>),
    Value(f64),
}

macro_rules! bin_operation {
    ($trait:ty, $fct_name:ident, $op:tt) => {
        impl $trait for Elem
        {
            type Output = Elem;
            fn $fct_name(self, other: Self::Output) -> Self::Output {
                match (self, other) {
                    (Elem::Image(lhs), Elem::Image(rhs)) => Elem::Image(lhs $op rhs),
                    (Elem::Value(lhs), Elem::Image(rhs)) => Elem::Image(rhs $op lhs),
                    (Elem::Image(lhs), Elem::Value(rhs)) => Elem::Image(lhs $op rhs),
                    (Elem::Value(lhs), Elem::Value(rhs)) => Elem::Value(lhs $op rhs),
                }
            }
        }
    }
}

bin_operation!(Add, add, +);
bin_operation!(Sub, sub, -);
bin_operation!(Mul, mul, *);
bin_operation!(Div, div, /);

pub trait Abs {
    type Output;

    #[must_use]
    fn abs(self) -> Self::Output;
}

impl Abs for Elem {
    type Output = Elem;

    fn abs(self) -> Elem {
        match self {
            Elem::Image(image) => Elem::Image(image.mapv_into(f64::abs)),
            Elem::Value(value) => Elem::Value(value.abs()),
        }
    }
}

pub fn read_3d_image<T>(path: &str) -> Array3<T>
where
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: DataElement,
    u8: AsPrimitive<T>,
    i8: AsPrimitive<T>,
    u16: AsPrimitive<T>,
    i16: AsPrimitive<T>,
    u32: AsPrimitive<T>,
    i32: AsPrimitive<T>,
    u64: AsPrimitive<T>,
    i64: AsPrimitive<T>,
    f32: AsPrimitive<T>,
    f64: AsPrimitive<T>,
{
    let nifti_object = InMemNiftiObject::from_file(path).expect("Nifti file is unreadable.");
    let volume = nifti_object.into_volume();
    let dyn_data = volume.into_ndarray::<T>().unwrap();
    dyn_data.into_dimensionality::<Ix3>().unwrap()
}

pub fn read_header(path: &str) -> NiftiHeader {
    let nifti_object = InMemNiftiObject::from_file(path).expect("Nifti file in unreadable.");
    nifti_object.header().clone()
}
