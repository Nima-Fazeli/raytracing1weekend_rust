// Implementing RayTracingInOneWeekend
// https://raytracing.github.io/books/RayTracingInOneWeekend.html
//
// Section 2. Output an Image
// The PPM Image Format
//

fn main() {
    // Image 

    let image_width:  u32 = 256;
    let image_height: u32 = 256;

    // Render

    println!("P3\n{0} {1}\n255\n", image_width, image_height);
    
    // I am going to predefine the variables so they don't get created in the for loop
    // not sure why the original blog post did the variable creation in the for loop
    let mut r: f32;
    let mut g: f32;
    let mut b: f32;

    let mut ir: u32;
    let mut ig: u32;
    let mut ib: u32;

    // For loop to generate pixel colors, row by row, left to right, top to bottom:

    for i in 0..256 {
        for j in 0..256 {
            r = (i as f32) / ((image_width as f32) - 1.0);
            g = (j as f32) / ((image_height as f32) - 1.0);
            b = 0.0;

            ir = (255.999 * r) as u32;
            ig = (255.999 * g) as u32;
            ib = (255.999 * b) as u32;

            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
