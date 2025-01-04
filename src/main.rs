// Implementing RayTracingInOneWeekend
// https://raytracing.github.io/books/RayTracingInOneWeekend.html
//
// Section 2. Output an Image
// The PPM Image Format
//
use std::fs::File;
use std::io::Write;

use indicatif::{ProgressBar};

pub mod vec3;

fn main() {
    let image_width:  u32 = 256;
    let image_height: u32 = 256;

    // Render

    // Create a new file
    let mut data_file = File::create("image.ppm").expect("creation failed");

    // write the first line to indicate the type of file/image format:
    data_file.write((format!("P3\n{0} {1}\n255\n", image_width, image_height)).as_bytes()).expect("write failed.");

    // I am going to predefine the variables so they don't get created in the for loop
    // not sure why the original blog post did the variable creation in the for loop
    let mut pixel_color_int = vec3::vec3::Color{ r: 0, g: 0, b: 0};

    let mut r: f32;
    let mut g: f32;
    let mut b: f32;

    let mut write_string;

    // For loop to generate pixel colors, row by row, left to right, top to bottom:
    // first we're going to make a progress bar
    let pb = ProgressBar::new(256);

    for i in 0..256 {
        // increment progress bar
        pb.inc(1);
        for j in 0..256 {
            r = (i as f32) / ((image_width as f32) - 1.0);
            g = (j as f32) / ((image_height as f32) - 1.0);
            b = 0.0;

            pixel_color_int.r = (255.999 * r) as u32;
            pixel_color_int.g = (255.999 * g) as u32;
            pixel_color_int.b = (255.999 * b) as u32;

            write_string = pixel_color_int.write_ppm();
            data_file.write(write_string.as_bytes()).expect("write failed.");
        }
    }

    pb.finish_with_message("image written.");
}
