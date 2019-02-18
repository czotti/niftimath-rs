mod elem;
mod operator;

#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use elem::*;
use nifti::writer::write_nifti;
use operator::Operator;

const USAGE: &'static str = "
Nifti math chaining mathematical operation defined in reverse polish notation.

Usage:
  niftimath [<elems>...] <output> [options]
  niftimath (-h | --help)

Informations:
  Example of command line usage, 
  niftimath t1.nii.gz 1.5 mul 0.3 add output.nii.gz

Dual operations:
  add   Addition take two operand
  div   Division take two operand
  mul   Multiplication take two operand
  sub   Substraction take two operand

Options:
  -d --datatype         Define in which datatype to save the result.
  -t --nb_thread=<t>    Use <t> cores to compute the math operation [default: 1].
  -h --help             Show this screen.
";

fn two_param(stack: &mut Vec<Elem>) -> (Elem, Elem) {
    let rhs = stack.pop().expect("Missing parameters lhs.");
    let lhs = stack.pop().expect("Missing parameters rhs.");
    (lhs, rhs)
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_output: String,
    arg_elems: Vec<String>,
}

fn main() {
    // let mut args: Vec<String> = env::args().collect();
    let args: Args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.deserialize())
        .unwrap_or_else(|e| e.exit());
    let mut header = None;
    println!("{:?}", args);
    let mut stack_data = vec![];
    for elem in args.arg_elems {
        match elem.parse::<f64>() {
            Ok(value) => stack_data.push(Elem::Value(value)),
            Err(_) => {
                let result = if elem.ends_with(".nii.gz") || elem.ends_with(".nii") {
                    if header.is_none() {
                        header = Some(read_header(&elem));
                    }
                    Elem::Image(read_3d_image(&elem))
                } else {
                    match elem.parse() {
                        Ok(Operator::Addition) => {
                            let (lhs, rhs) = two_param(&mut stack_data);
                            lhs + rhs
                        }
                        Ok(Operator::Division) => {
                            let (lhs, rhs) = two_param(&mut stack_data);
                            lhs / rhs
                        }
                        Ok(Operator::Multiplication) => {
                            let (lhs, rhs) = two_param(&mut stack_data);
                            lhs * rhs
                        }
                        Ok(Operator::Substraction) => {
                            let (lhs, rhs) = two_param(&mut stack_data);
                            lhs - rhs
                        }
                        Ok(Operator::Absolute) => stack_data.pop().unwrap().abs(),
                        Ok(Operator::Floor) => stack_data.pop().unwrap().floor(),
                        Ok(Operator::Ceil) => stack_data.pop().unwrap().ceil(),
                        Ok(Operator::Round) => stack_data.pop().unwrap().round(),
                        Ok(Operator::Sqrt) => stack_data.pop().unwrap().sqrt(),
                        Ok(Operator::Cbrt) => stack_data.pop().unwrap().cbrt(),
                        Ok(Operator::Exp) => stack_data.pop().unwrap().exp(),
                        Ok(Operator::Exp2) => stack_data.pop().unwrap().exp2(),
                        Ok(Operator::Ln) => stack_data.pop().unwrap().ln(),
                        Ok(Operator::Log2) => stack_data.pop().unwrap().log2(),
                        Ok(Operator::Log10) => stack_data.pop().unwrap().log10(),
                        Ok(Operator::Sin) => stack_data.pop().unwrap().sin(),
                        Ok(Operator::Cos) => stack_data.pop().unwrap().cos(),
                        Ok(Operator::Tan) => stack_data.pop().unwrap().tan(),
                        Ok(Operator::Asin) => stack_data.pop().unwrap().asin(),
                        Ok(Operator::Acos) => stack_data.pop().unwrap().acos(),
                        Ok(Operator::Atan) => stack_data.pop().unwrap().atan(),
                        Ok(Operator::Sinh) => stack_data.pop().unwrap().sinh(),
                        Ok(Operator::Cosh) => stack_data.pop().unwrap().cosh(),
                        Ok(Operator::Tanh) => stack_data.pop().unwrap().tanh(),
                        Err(e) => panic!(e),
                    }
                };
                stack_data.push(result);
            }
        }
    }
    let image = match stack_data.pop().unwrap() {
        Elem::Image(image) => image,
        _ => panic!("The latest computed value is not an image."),
    };

    let header = header.unwrap();
    write_nifti(args.arg_output, &image, Some(&header)).expect("Failed to save the image.");
}
