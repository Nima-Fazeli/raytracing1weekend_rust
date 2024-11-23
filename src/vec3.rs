// create a vector class with some printing capabilities

pub mod vec3 {
    use std::ops::{Add, Sub};
    
    pub struct Color(pub u32, pub u32, pub u32);

    impl Color {
        // function to write ppm
        pub fn write_ppm(&self) -> String {
            let hello = String::from(format!("{0} {1} {2}\n", self.0, self.1, self.2));
            return hello;
        }
    }

    impl Add<Color> for Color {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {0: self.0 + other.0, 1: self.1 + other.1, 2: self.2 + other.2}
        }
    }
    

    impl Sub<Color> for Color {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self {0: self.0 - other.0, 1: self.1 - other.1, 2: self.2 - other.2}
        }
    }

    impl Copy for Color {}

    impl Clone for Color {
        fn clone(&self) -> Self {
            *self 
        }
    }
}
