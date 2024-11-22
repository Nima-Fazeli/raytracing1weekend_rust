// create a vector class with some printing capabilities

pub mod vec3 {

    pub struct Color(pub u32, pub u32, pub u32);

    impl Color {
        // function to write ppm
        pub fn write_ppm(&self) -> String {
            let hello = String::from(format!("{0} {1} {2}\n", self.0, self.1, self.2));
            return hello;
        }
    }
}
