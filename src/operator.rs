use std::str::FromStr;

pub enum Operator {
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
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Operator, String> {
        match s {
            "add" => Ok(Operator::Addition),
            "sub" => Ok(Operator::Substraction),
            "mul" => Ok(Operator::Multiplication),
            "div" => Ok(Operator::Division),
            "abs" => Ok(Operator::Absolute),
            "floor" => Ok(Operator::Floor),
            "ceil" => Ok(Operator::Ceil),
            "round" => Ok(Operator::Round),
            "sqrt" => Ok(Operator::Sqrt),
            "cbrt" => Ok(Operator::Cbrt),
            "exp" => Ok(Operator::Exp),
            "exp2" => Ok(Operator::Exp2),
            "ln" => Ok(Operator::Ln),
            "log2" => Ok(Operator::Log2),
            "log10" => Ok(Operator::Log10),
            "sin" => Ok(Operator::Sin),
            "cos" => Ok(Operator::Cos),
            "tan" => Ok(Operator::Tan),
            "asin" => Ok(Operator::Asin),
            "acos" => Ok(Operator::Acos),
            "atan" => Ok(Operator::Atan),
            "sinh" => Ok(Operator::Sinh),
            "cosh" => Ok(Operator::Cosh),
            "tanh" => Ok(Operator::Tanh),
            _ => Err(format!("Operator not implemented: {:?}", s)),
        }
    }
}
