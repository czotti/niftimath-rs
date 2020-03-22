use std::str::FromStr;

pub enum Formula {
    Image(String),
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
                Ok(Formula::Image(image.to_string()))
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
