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
This correspond to the following operation:
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

### Binary Operation
- [X] Addition (add)
- [X] Subtraction (sub)
- [X] Multiplication (mul)
- [X] Division (div)
### Unary Opetation
- [ ] Minimum (min), between images will be a min voxel wise.
- [ ] Maximum (max), between images will be a max voxel wise.
- [X] Absolute (abs), voxel wise

### Reduce operation
- [ ] Mean (mean), compute the mean of all the voxel of the image
- [ ] Standard deviation (std), compute the std of all the voxel of the image.

### Output operation
- [ ] Add possibility to save as a desired nifti type (u8, u16, f32, f64, ...)