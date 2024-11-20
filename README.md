# Raytracing in 1 Weekend -- Implemented in Rust


This repository is an reimplementation of [this awesome blog post](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
on raytracing in Rust. I am mostly interested in learning Rust and this
is a great way to do it.

To run files in this repository, you need a Rust compiler. You can install Rust any number of way. On my MacOS, I did the following:

- [Install and setup miniforge](https://kirenz.github.io/codelabs/codelabs/miniforge-setup/#0) (make sure to remove Anaconda otherwise things get squirrely)
- [Setup a virtual environment and install Rust with it](https://www.howtoforge.com/how-to-create-rust-virtual-environment-using-conda-on-linux/) 

Once you have the environment setup, you can clone this repository and compile source codes. For example:

```
rustc create_ppm_images.rs 
```

Then run the executable ```./create_ppm_images```
