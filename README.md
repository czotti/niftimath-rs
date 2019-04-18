# niftimath-rs

NiftiMath rust project.

This project provides a simple way to do mathematical operation on nifti images the 
same way mrcalc does in [mrtrix3](https://github.com/MRtrix3/mrtrix3)).

The formulas are expressed in [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation).

## Exemple
Binary operation (Mul, Div, Sub, Add)
```
niftimath 1.0 t1.nii.gz mul 3 6.1 add mul output.nii.gz
```
This correspond to the following formula:
```
output.nii.gz = (1.0 * t1.nii.gz) * (3 + 6.1)
```

Unary Operation (Abs, etc)
``` 
niftimath 5.0 t1.nii.gz sub abs 10 mul output.nii.gz
```
This is equivalent to:
```
output.nii.gz = abs(5.0 - t1.nii.gz) * 10
```

## Operations

### Binary Operations
- [X] Addition (add).
- [X] Subtraction (sub).
- [X] Multiplication (mul).
- [X] Division (div).
- [ ] Minimum (min), between images will be a min voxels wise.
- [ ] Maximum (max), between images will be a max voxels wise.

### Unary Operations
- [ ] Implement unary [operations](https://doc.rust-lang.org/std/primitive.f64.html), voxels wise.
    - [X] Absolute (abs), voxels wise.
    - [X] Floor, Ceil, Round.
    - [X] Sqrt, Cbrt.
    - [X] Exp, Exp2, Ln, Log2, Log10.
    - [X] Sin, Cos, Tan, Asin, Acos, Atan, Sinh, Cosh, Tanh.
    - [ ] LogN.
    - [ ] Powi, Powf.

### Reduce operations
- [ ] Minimum (min_reduce), extract minimum value of an image.
- [ ] Maximum (max_reduce), extract maximum value of an image.
- [ ] Mean (mean_reduce), compute the mean of all the voxels of the image.
- [ ] Standard deviation (std_reduce), compute the std of all the voxels of the image.

### Output operations
- [X] Add possibility to save as a desired nifti type (u8, u16, f32, f64, ...).

### Features
- [X] Parallel iteration for some operations (ndarray-parallel).
- [X] Cache for input image to now reload it every time.