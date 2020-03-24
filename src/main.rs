mod operator;
mod utils;

use docopt::Docopt;
use nifti::writer::write_nifti;
use operator::*;
use serde_derive::Deserialize;
use std::collections::HashMap;
use utils::{read_header, set_threading};

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

    let mut ccache = HashMap::new();
    println!("{:?}", args);
    let mut stack_data = vec![];
    let mut header = None;
    for elem in args.arg_elems {
        if header.is_none() && elem.ends_with(".nii.gz") || elem.ends_with(".nii") {
            header = Some(read_header(&elem));
        }
        let result = elem
            .parse::<Formula>()
            .unwrap()
            .apply(&mut stack_data, &mut ccache);
        stack_data.push(result);
    }
    let image = match stack_data.pop().unwrap() {
        Formula::Image(image) => image,
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
