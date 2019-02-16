# niftimath-rs

NiftiMath rust project.

This project provide a simple way to make mathematical operation on nifti images (the 
same way mrcalc from [mrtrix3](https://github.com/MRtrix3/mrtrix3)).

The formulas are express in [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation).

## Exemple
```
niftimath 1.0 t1.nii.gz mul 3 6.1 add mul output.nii.gz
```
This correspond to the following operation:
```
output.nii.gz = (1.0 * t1.nii.gz) * (3 + 6.1)
``` 

## Operations

- [X] Addition (add)
- [X] Subtraction (sub)
- [X] Multiplication (mul)
- [X] Division (div)
- [ ] Add possibility to save as a desired nifti type (u8, u16, f32, f64, ...)
- [ ] Minimum (min), between images will be a min voxel wise.
- [ ] Maximum (max), between images will be a max voxel wise.
- [X] Absolute (abs), voxel wise
- [ ] Mean (mean), compute the mean of all the voxel of the image
- [ ] Standard deviation (std), compute the std of all the voxel of the image.