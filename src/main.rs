mod elem;
mod operator;
mod utils;

use count_cache::CountCache;
use docopt::Docopt;
use elem::*;
use nifti::writer::write_nifti;
use operator::Formula;
use serde_derive::Deserialize;
use utils::{read_header, read_nd_image, set_threading};

const USAGE: &'static str = "
Nifti math chaining mathematical operation defined in reverse polish notation.

Usage:
  niftimath [<elems>...] <output> [options]
  niftimath (-h | --help)

Informations:
  Example of command line usage, 
  niftimath t1.nii.gz 1.5 mul 0.3 add output.nii.gz

Binary operations (voxel wise for images):
  add   Addition take two operand
  div   Division take two operand
  mul   Multiplication take two operand
  sub   Substraction take two operand

Unary operations (voxel wise for images):
  abs
  floor
  ceil
  round
  sqrt
  cbrt
  exp
  exp2
  ln
  log2
  log10
  sin
  cos
  tan
  asin
  acos
  atan
  sinh
  cosh
  tanh

Save types:
    u8, i8, u16, i16, u32, i32, u64, i64, f32, f64

Options:
  -d --datatype=<d>     Define in which datatype to save the result [default: f64].
  -t --threads=<t>   Use <t> cores to compute the math operation [default: 1].
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
    flag_datatype: String,
    flag_threads: usize,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.deserialize())
        .unwrap_or_else(|e| e.exit());
    set_threading(args.flag_threads);

    let mut header = None;
    let mut ccache = CountCache::new();
    for elem in args.arg_elems.iter() {
        match elem.parse() {
            Ok(Formula::Image(image_path)) => {
                if ccache.contains_key(&image_path) {
                    ccache.increment(&image_path, 1);
                } else {
                    if header.is_none() {
                        header = Some(read_header(&image_path));
                    }
                    ccache.insert(image_path.clone(), read_nd_image(image_path), 1)
                }
            }
            _ => (),
        }
    }
    println!("{:?}", args);
    let mut stack_data = vec![];
    for elem in args.arg_elems {
        let result = match elem.parse() {
            Ok(Formula::Value(value)) => Elem::Value(value),
            Ok(Formula::Image(image)) => {
                Elem::Image(ccache.get(&image).expect("Failing to retrieve image"))
            }
            Ok(Formula::Addition) => {
                let (lhs, rhs) = two_param(&mut stack_data);
                lhs + rhs
            }
            Ok(Formula::Division) => {
                let (lhs, rhs) = two_param(&mut stack_data);
                lhs / rhs
            }
            Ok(Formula::Multiplication) => {
                let (lhs, rhs) = two_param(&mut stack_data);
                lhs * rhs
            }
            Ok(Formula::Substraction) => {
                let (lhs, rhs) = two_param(&mut stack_data);
                lhs - rhs
            }
            Ok(Formula::Absolute) => stack_data.pop().unwrap().abs(),
            Ok(Formula::Floor) => stack_data.pop().unwrap().floor(),
            Ok(Formula::Ceil) => stack_data.pop().unwrap().ceil(),
            Ok(Formula::Round) => stack_data.pop().unwrap().round(),
            Ok(Formula::Sqrt) => stack_data.pop().unwrap().sqrt(),
            Ok(Formula::Cbrt) => stack_data.pop().unwrap().cbrt(),
            Ok(Formula::Exp) => stack_data.pop().unwrap().exp(),
            Ok(Formula::Exp2) => stack_data.pop().unwrap().exp2(),
            Ok(Formula::Ln) => stack_data.pop().unwrap().ln(),
            Ok(Formula::Log2) => stack_data.pop().unwrap().log2(),
            Ok(Formula::Log10) => stack_data.pop().unwrap().log10(),
            Ok(Formula::Sin) => stack_data.pop().unwrap().sin(),
            Ok(Formula::Cos) => stack_data.pop().unwrap().cos(),
            Ok(Formula::Tan) => stack_data.pop().unwrap().tan(),
            Ok(Formula::Asin) => stack_data.pop().unwrap().asin(),
            Ok(Formula::Acos) => stack_data.pop().unwrap().acos(),
            Ok(Formula::Atan) => stack_data.pop().unwrap().atan(),
            Ok(Formula::Sinh) => stack_data.pop().unwrap().sinh(),
            Ok(Formula::Cosh) => stack_data.pop().unwrap().cosh(),
            Ok(Formula::Tanh) => stack_data.pop().unwrap().tanh(),
            Err(e) => panic!(e),
        };
        stack_data.push(result);
    }
    let image = match stack_data.pop().unwrap() {
        Elem::Image(image) => image,
        _ => panic!("The latest computed value is not an image."),
    };

    let header = header.unwrap();
    let error_convert = "Failed to save the image.";
    match args.flag_datatype.as_ref() {
        "u8" => {
            let image = image.mapv(|e| e.round() as u8);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "i8" => {
            let image = image.mapv(|e| e.round() as i8);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "u16" => {
            let image = image.mapv(|e| e.round() as u16);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "i16" => {
            let image = image.mapv(|e| e.round() as i16);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "u32" => {
            let image = image.mapv(|e| e.round() as u32);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "i32" => {
            let image = image.mapv(|e| e.round() as i32);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "i64" => {
            let image = image.mapv(|e| e.round() as i64);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "u64" => {
            let image = image.mapv(|e| e.round() as u64);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "f32" => {
            let image = image.mapv(|e| e as f32);
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        "f64" => {
            write_nifti(args.arg_output, &image, Some(&header)).expect(error_convert);
        }
        _ => panic!("Unsupported type."),
    }
}
