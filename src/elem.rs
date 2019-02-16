use ndarray::{Array3, Ix3, ScalarOperand};
use nifti::{DataElement, InMemNiftiObject, IntoNdArray, NiftiHeader, NiftiObject};
use num_traits::AsPrimitive;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum Elem<T> {
    Image(Array3<T>),
    Value(T),
}

impl<T> Add for Elem<T>
where
    T: ScalarOperand + Add<Output = T>,
{
    type Output = Elem<T>;

    fn add(self, other: Elem<T>) -> Elem<T> {
        match (self, other) {
            (Elem::Image(lhs), Elem::Image(rhs)) => Elem::Image(lhs + rhs),
            (Elem::Value(lhs), Elem::Image(rhs)) => Elem::Image(rhs + lhs),
            (Elem::Image(lhs), Elem::Value(rhs)) => Elem::Image(lhs + rhs),
            (Elem::Value(lhs), Elem::Value(rhs)) => Elem::Value(lhs + rhs),
        }
    }
}

impl<T> Sub for Elem<T>
where
    T: ScalarOperand + Sub<Output = T>,
{
    type Output = Elem<T>;

    fn sub(self, other: Elem<T>) -> Elem<T> {
        match (self, other) {
            (Elem::Image(lhs), Elem::Image(rhs)) => Elem::Image(lhs - rhs),
            (Elem::Value(lhs), Elem::Image(rhs)) => Elem::Image(rhs - lhs),
            (Elem::Image(lhs), Elem::Value(rhs)) => Elem::Image(lhs - rhs),
            (Elem::Value(lhs), Elem::Value(rhs)) => Elem::Value(lhs - rhs),
        }
    }
}

impl<T> Mul for Elem<T>
where
    T: ScalarOperand + Mul<Output = T>,
{
    type Output = Elem<T>;

    fn mul(self, other: Elem<T>) -> Elem<T> {
        match (self, other) {
            (Elem::Image(lhs), Elem::Image(rhs)) => Elem::Image(lhs * rhs),
            (Elem::Value(lhs), Elem::Image(rhs)) => Elem::Image(rhs * lhs),
            (Elem::Image(lhs), Elem::Value(rhs)) => Elem::Image(lhs * rhs),
            (Elem::Value(lhs), Elem::Value(rhs)) => Elem::Value(lhs * rhs),
        }
    }
}

impl<T> Div for Elem<T>
where
    T: ScalarOperand + Div<Output = T>,
{
    type Output = Elem<T>;

    fn div(self, other: Elem<T>) -> Elem<T> {
        match (self, other) {
            (Elem::Image(lhs), Elem::Image(rhs)) => Elem::Image(lhs / rhs),
            (Elem::Value(lhs), Elem::Image(rhs)) => Elem::Image(rhs / lhs),
            (Elem::Image(lhs), Elem::Value(rhs)) => Elem::Image(lhs / rhs),
            (Elem::Value(lhs), Elem::Value(rhs)) => Elem::Value(lhs / rhs),
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
