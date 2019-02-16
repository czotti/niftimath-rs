use std::str::FromStr;

pub enum Operator {
    Addition,
    Division,
    Multiplication,
    Substraction,
    Absolute,
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
            _ => Err(format!("Operator not implemented: {:?}", s)),
        }
    }
}
