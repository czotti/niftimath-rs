use crate::utils::{extract_volume, read_nifti};
use approx::abs_diff_eq;
use ndarray::{parallel::par_azip, Array, IxDyn};
use nifti::InMemNiftiObject;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Debug)]
pub enum Formula {
    ImagePath(String),
    Image(Array<f64, IxDyn>),
    Value(f64),
    Addition,
    Division,
    Multiplication,
    Substraction,
    Absolute,
    Floor,
    Ceil,
    Round,
    Sqrt,
    Cbrt,
    Exp,
    Exp2,
    Ln,
    Log2,
    Log10,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    // Reduce operation
    ReduceMin,
    ReduceMax,
    ReduceMean,
    ReduceStd,
    ReduceMedian,
}

impl FromStr for Formula {
    type Err = String;

    fn from_str(s: &str) -> Result<Formula, String> {
        match s {
            image if image.ends_with(".nii.gz") || image.ends_with(".nii") => {
                Ok(Formula::ImagePath(image.to_string()))
            }
            value if value.parse::<f64>().is_ok() => {
                Ok(Formula::Value(value.parse::<f64>().unwrap()))
            }
            "add" => Ok(Formula::Addition),
            "sub" => Ok(Formula::Substraction),
            "mul" => Ok(Formula::Multiplication),
            "div" => Ok(Formula::Division),
            "abs" => Ok(Formula::Absolute),
            "floor" => Ok(Formula::Floor),
            "ceil" => Ok(Formula::Ceil),
            "round" => Ok(Formula::Round),
            "sqrt" => Ok(Formula::Sqrt),
            "cbrt" => Ok(Formula::Cbrt),
            "exp" => Ok(Formula::Exp),
            "exp2" => Ok(Formula::Exp2),
            "ln" => Ok(Formula::Ln),
            "log2" => Ok(Formula::Log2),
            "log10" => Ok(Formula::Log10),
            "sin" => Ok(Formula::Sin),
            "cos" => Ok(Formula::Cos),
            "tan" => Ok(Formula::Tan),
            "asin" => Ok(Formula::Asin),
            "acos" => Ok(Formula::Acos),
            "atan" => Ok(Formula::Atan),
            "sinh" => Ok(Formula::Sinh),
            "cosh" => Ok(Formula::Cosh),
            "tanh" => Ok(Formula::Tanh),
            "reduce_mean" => Ok(Formula::ReduceMean),
            "reduce_median" => Ok(Formula::ReduceMedian),
            "reduce_std" => Ok(Formula::ReduceStd),
            "reduce_min" => Ok(Formula::ReduceMin),
            "reduce_max" => Ok(Formula::ReduceMax),
            _ => Err(format!("Formula not implemented: {:?}", s)),
        }
    }
}

impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Formula::Value(lhs), Formula::Value(rhs)) => lhs == rhs,
            (Formula::Image(lhs), Formula::Image(rhs)) => abs_diff_eq!(lhs, rhs),
            (_, _) => false,
        }
    }
}

impl Formula {
    pub fn apply(
        self,
        stack: &mut Vec<Formula>,
        ccache: &mut HashMap<String, InMemNiftiObject>,
    ) -> Formula {
        match self {
            Formula::Value(value) => Formula::Value(value),
            Formula::ImagePath(image_path) => {
                if !ccache.contains_key(&image_path) {
                    ccache.insert(image_path.clone(), read_nifti(&image_path));
                }
                Formula::Image(extract_volume(
                    ccache
                        .get(&image_path)
                        .expect("Failing to retrieve image")
                        .clone(),
                ))
            }
            Formula::Image(image) => Formula::Image(image),
            Formula::Addition => {
                let rhs = stack.pop().expect("Missing parameters lhs.");
                let lhs = stack.pop().expect("Missing parameters rhs.");
                lhs + rhs
            }
            Formula::Division => {
                let rhs = stack.pop().expect("Missing parameters lhs.");
                let lhs = stack.pop().expect("Missing parameters rhs.");
                lhs / rhs
            }
            Formula::Multiplication => {
                let rhs = stack.pop().expect("Missing parameters lhs.");
                let lhs = stack.pop().expect("Missing parameters rhs.");
                lhs * rhs
            }
            Formula::Substraction => {
                let rhs = stack.pop().expect("Missing parameters lhs.");
                let lhs = stack.pop().expect("Missing parameters rhs.");
                lhs - rhs
            }
            Formula::Absolute => stack.pop().unwrap().abs(),
            Formula::Floor => stack.pop().unwrap().floor(),
            Formula::Ceil => stack.pop().unwrap().ceil(),
            Formula::Round => stack.pop().unwrap().round(),
            Formula::Sqrt => stack.pop().unwrap().sqrt(),
            Formula::Cbrt => stack.pop().unwrap().cbrt(),
            Formula::Exp => stack.pop().unwrap().exp(),
            Formula::Exp2 => stack.pop().unwrap().exp2(),
            Formula::Ln => stack.pop().unwrap().ln(),
            Formula::Log2 => stack.pop().unwrap().log2(),
            Formula::Log10 => stack.pop().unwrap().log10(),
            Formula::Sin => stack.pop().unwrap().sin(),
            Formula::Cos => stack.pop().unwrap().cos(),
            Formula::Tan => stack.pop().unwrap().tan(),
            Formula::Asin => stack.pop().unwrap().asin(),
            Formula::Acos => stack.pop().unwrap().acos(),
            Formula::Atan => stack.pop().unwrap().atan(),
            Formula::Sinh => stack.pop().unwrap().sinh(),
            Formula::Cosh => stack.pop().unwrap().cosh(),
            Formula::Tanh => stack.pop().unwrap().tanh(),
        }
    }
}

macro_rules! apply_unary {
    ($trait:ident, $fct_name:ident, $op:path) => {
        pub trait $trait {
            type Output;
            fn $fct_name(self) -> Self::Output;
        }
        impl $trait for Formula {
            type Output = Formula;
            fn $fct_name(self) -> Formula {
                match self {
                    Formula::Image(mut image) => {
                        image.par_mapv_inplace($op);
                        Formula::Image(image)
                    }
                    Formula::Value(value) => Formula::Value($op(value)),
                    _ => panic!("Should not be called!"),
                }
            }
        }
    };
}

apply_unary!(Abs, abs, f64::abs);
apply_unary!(Floor, floor, f64::floor);
apply_unary!(Ceil, ceil, f64::ceil);
apply_unary!(Round, round, f64::round);
apply_unary!(Sqrt, sqrt, f64::sqrt);
apply_unary!(Cbrt, cbrt, f64::cbrt);
apply_unary!(Exp, exp, f64::exp);
apply_unary!(Exp2, exp2, f64::exp2);
apply_unary!(Ln, ln, f64::ln);
apply_unary!(Log2, log2, f64::log2);
apply_unary!(Log10, log10, f64::log10);
apply_unary!(Sin, sin, f64::sin);
apply_unary!(Cos, cos, f64::cos);
apply_unary!(Tan, tan, f64::tan);
apply_unary!(Asin, asin, f64::asin);
apply_unary!(Acos, acos, f64::acos);
apply_unary!(Atan, atan, f64::atan);
apply_unary!(Sinh, sinh, f64::sinh);
apply_unary!(Cosh, cosh, f64::cosh);
apply_unary!(Tanh, tanh, f64::tanh);

macro_rules! apply_dyadic {
    ($trait:ty, $fct_name:ident, $op:tt) => {
        impl $trait for Formula
        {
            type Output = Formula;
            fn $fct_name(self, other: Self::Output) -> Self::Output {
                // TODO: add broadcasting when lhs or rhs as different # dims.
                match (self, other) {
                    (Formula::Image(mut lhs), Formula::Image(rhs)) => {
                        par_azip!((lhs in &mut lhs, &rhs in &rhs) { *lhs $op rhs });
                        Formula::Image(lhs)
                    },
                    (Formula::Value(lhs), Formula::Image(mut rhs)) => {
                        rhs.par_map_inplace(|e| *e $op lhs);
                        Formula::Image(rhs)
                    },
                    (Formula::Image(mut lhs), Formula::Value(rhs)) => {
                        lhs.par_map_inplace(|e| *e $op rhs);
                        Formula::Image(lhs)
                    },
                    (Formula::Value(mut lhs), Formula::Value(rhs)) => {
                        lhs $op rhs;
                        Formula::Value(lhs)
                    },
                    (_, _) => panic!("Operation not supported for the other types.")
                }
            }
        }
    }
}

apply_dyadic!(Add, add, +=);
apply_dyadic!(Sub, sub, -=);
apply_dyadic!(Mul, mul, *=);
apply_dyadic!(Div, div, /=);

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_dyadic_add_values() {
        let lhs = Formula::Value(20.0);
        let rhs = Formula::Value(10.0);
        assert_eq!(Formula::Value(30.0), lhs + rhs);
    }

    #[test]
    fn test_dyadic_sub_values() {
        let lhs = Formula::Value(20.0);
        let rhs = Formula::Value(10.0);
        assert_eq!(Formula::Value(10.0), lhs - rhs);
    }

    #[test]
    fn test_dyadic_mul_values() {
        let lhs = Formula::Value(20.0);
        let rhs = Formula::Value(10.0);
        assert_eq!(Formula::Value(200.0), lhs * rhs);
    }

    #[test]
    fn test_dyadic_div_values() {
        let lhs = Formula::Value(20.0);
        let rhs = Formula::Value(10.0);
        assert_eq!(Formula::Value(2.0), lhs / rhs);
    }

    fn get_ndarray() -> (Array2<f64>, Array2<f64>) {
        let arr = Array::range(0., 16., 1.).into_shape((4, 4)).unwrap();
        (arr, Array2::from_elem((4, 4), 2.))
    }

    #[test]
    fn test_dyadic_add_ndarray() {
        let (nd_lhs, nd_rhs) = get_ndarray();
        let lhs = Formula::Image(nd_lhs.clone().into_dyn());
        let rhs = Formula::Image(nd_rhs.clone().into_dyn());
        assert_eq!(Formula::Image((nd_lhs + nd_rhs).into_dyn()), lhs + rhs);
    }

    #[test]
    fn test_dyadic_sub_ndarray() {
        let (nd_lhs, nd_rhs) = get_ndarray();
        let lhs = Formula::Image(nd_lhs.clone().into_dyn());
        let rhs = Formula::Image(nd_rhs.clone().into_dyn());
        assert_eq!(Formula::Image((nd_lhs - nd_rhs).into_dyn()), lhs - rhs);
    }

    #[test]
    fn test_dyadic_mul_ndarray() {
        let (nd_lhs, nd_rhs) = get_ndarray();
        let lhs = Formula::Image(nd_lhs.clone().into_dyn());
        let rhs = Formula::Image(nd_rhs.clone().into_dyn());
        assert_eq!(Formula::Image((nd_lhs * nd_rhs).into_dyn()), lhs * rhs);
    }

    #[test]
    fn test_dyadic_div_ndarray() {
        let (nd_lhs, nd_rhs) = get_ndarray();
        let lhs = Formula::Image(nd_lhs.clone().into_dyn());
        let rhs = Formula::Image(nd_rhs.clone().into_dyn());
        assert_eq!(Formula::Image((nd_lhs / nd_rhs).into_dyn()), lhs / rhs);
    }

    #[test]
    fn test_dyadic_add_ndarray_value() {
        let (nd_lhs, nd_rhs) = get_ndarray();
        let lhs = Formula::Image(nd_lhs.clone().into_dyn());
        let rhs = Formula::Image(nd_rhs.clone().into_dyn());
        assert_eq!(
            Formula::Image((2.0 + nd_rhs).into_dyn()),
            Formula::Value(2.0) + rhs
        );
        assert_eq!(
            Formula::Image((nd_lhs + 2.0).into_dyn()),
            lhs + Formula::Value(2.0)
        );
    }

    #[test]
    fn test_dyadic_sub_ndarray_value() {
        let (nd_lhs, nd_rhs) = get_ndarray();
        let lhs = Formula::Image(nd_lhs.clone().into_dyn());
        let rhs = Formula::Image(nd_rhs.clone().into_dyn());
        assert_eq!(
            Formula::Image((2.0 - nd_rhs).into_dyn()),
            Formula::Value(2.0) - rhs
        );
        assert_eq!(
            Formula::Image((nd_lhs - 2.0).into_dyn()),
            lhs - Formula::Value(2.0)
        );
    }

    #[test]
    fn test_dyadic_mul_ndarray_value() {
        let (nd_lhs, nd_rhs) = get_ndarray();
        let lhs = Formula::Image(nd_lhs.clone().into_dyn());
        let rhs = Formula::Image(nd_rhs.clone().into_dyn());
        assert_eq!(
            Formula::Image((2.0 * nd_rhs).into_dyn()),
            Formula::Value(2.0) * rhs
        );
        assert_eq!(
            Formula::Image((nd_lhs * 2.0).into_dyn()),
            lhs * Formula::Value(2.0)
        );
    }

    #[test]
    fn test_dyadic_div_ndarray_value() {
        let (nd_lhs, nd_rhs) = get_ndarray();
        let lhs = Formula::Image(nd_lhs.clone().into_dyn());
        let rhs = Formula::Image(nd_rhs.clone().into_dyn());
        assert_eq!(
            Formula::Image((2.0 / nd_rhs).into_dyn()),
            Formula::Value(2.0) / rhs
        );
        assert_eq!(
            Formula::Image((nd_lhs / 2.0).into_dyn()),
            lhs / Formula::Value(2.0)
        );
    }
}
