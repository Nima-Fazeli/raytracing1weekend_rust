# Raytracing in 1 Weekend -- Implemented in Rust


This repository is an reimplementation of [this awesome blog post](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
on raytracing in Rust. I am mostly interested in learning Rust and this
is a great way to do it.

To run files in this repository, you need a Rust compiler. You can install Rust any number of way. On my MacOS, I did the following:

- [Install and setup miniforge](https://kirenz.github.io/codelabs/codelabs/miniforge-setup/#0) (make sure to remove Anaconda otherwise things get squirrely)
- [Setup a virtual environment and install Rust with it](https://www.howtoforge.com/how-to-create-rust-virtual-environment-using-conda-on-linux/) 

Once you have the environment setup, you can clone this repository and compile source codes. The project structure is designed to be built with [Cargo](https://doc.rust-lang.org/cargo/). Thus to build the project, you can type


```
Cargo build 
```

This will compile the contents of ```main.rs''' into ```./target/debug/this_string``` where the binary will be named according to:

```
[[bin]]
name = "this_string"
```

### Coding Liberties

In re-implementing the blog post, I've decided to take some liberties:

- I am using ```indicatif``` for progress bars. Here is the [repo](https://github.com/console-rs/indicatif/tree/632989d04e63a6bbdac8d89702c51009999dc81c)
