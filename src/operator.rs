use std::str::FromStr;

pub enum Operator {
    Addition,
    Division,
    Multiplication,
    Substraction,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Operator, String> {
        match s {
            "add" => Ok(Operator::Addition),
            "sub" => Ok(Operator::Substraction),
            "mul" => Ok(Operator::Multiplication),
            "div" => Ok(Operator::Division),
            _ => Err(format!("Operator not implemented: {:?}", s)),
        }
    }
}
