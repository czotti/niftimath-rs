use ndarray::{Array3, Ix3};
use ndarray_parallel::{par_azip, prelude::*};
use nifti::{InMemNiftiObject, IntoNdArray, NiftiHeader, NiftiObject};
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
                    (Elem::Image(mut lhs), Elem::Image(rhs)) => {
                        par_azip!(mut lhs, rhs in {
                            *lhs $op rhs
                        });
                        Elem::Image(lhs)
                    },
                    (Elem::Value(lhs), Elem::Image(mut rhs)) => {
                        rhs.par_map_inplace(|e| *e $op lhs);
                        Elem::Image(rhs)
                    },
                    (Elem::Image(mut lhs), Elem::Value(rhs)) => {
                        lhs.par_map_inplace(|e| *e $op rhs);
                        Elem::Image(lhs)
                    },
                    (Elem::Value(mut lhs), Elem::Value(rhs)) => {
                        lhs $op rhs;
                        Elem::Value(lhs)
                    },
                }
            }
        }
    }
}

bin_operation!(Add, add, +=);
bin_operation!(Sub, sub, -=);
bin_operation!(Mul, mul, *=);
bin_operation!(Div, div, /=);

macro_rules! unary_operation {
    ($trait:ident, $fct_name:ident, $op:path) => {
        pub trait $trait {
            type Output;
            fn $fct_name(self) -> Self::Output;
        }

        impl $trait for Elem {
            type Output = Elem;
            fn $fct_name(self) -> Elem {
                match self {
                    Elem::Image(mut image) => {
                        image.par_mapv_inplace($op);
                        Elem::Image(image)
                    }
                    Elem::Value(value) => Elem::Value($op(value)),
                }
            }
        }
    };
}

unary_operation!(Abs, abs, f64::abs);
unary_operation!(Floor, floor, f64::floor);
unary_operation!(Ceil, ceil, f64::ceil);
unary_operation!(Round, round, f64::round);
unary_operation!(Sqrt, sqrt, f64::sqrt);
unary_operation!(Cbrt, cbrt, f64::cbrt);
unary_operation!(Exp, exp, f64::exp);
unary_operation!(Exp2, exp2, f64::exp2);
unary_operation!(Ln, ln, f64::ln);
unary_operation!(Log2, log2, f64::log2);
unary_operation!(Log10, log10, f64::log10);
unary_operation!(Sin, sin, f64::sin);
unary_operation!(Cos, cos, f64::cos);
unary_operation!(Tan, tan, f64::tan);
unary_operation!(Asin, asin, f64::asin);
unary_operation!(Acos, acos, f64::acos);
unary_operation!(Atan, atan, f64::atan);
unary_operation!(Sinh, sinh, f64::sinh);
unary_operation!(Cosh, cosh, f64::cosh);
unary_operation!(Tanh, tanh, f64::tanh);

pub fn read_3d_image(path: &str) -> Array3<f64> {
    let nifti_object = InMemNiftiObject::from_file(path).expect("Nifti file is unreadable.");
    let volume = nifti_object.into_volume();
    let dyn_data = volume.into_ndarray::<f64>().unwrap();
    dyn_data.into_dimensionality::<Ix3>().unwrap()
}

pub fn read_header(path: &str) -> NiftiHeader {
    let nifti_object = InMemNiftiObject::from_file(path).expect("Nifti file in unreadable.");
    nifti_object.header().clone()
}
