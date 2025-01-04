// create a vector class with some printing capabilities

pub mod vec3 {
    use std::ops::{Add, Sub, Mul, Neg, Div};

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Color {
        pub r: u32,
        pub g: u32,
        pub b: u32,
    }

    impl Color {
        // function to write ppm
        pub fn write_ppm(&self) -> String {
            let hello = String::from(
                format!("{0} {1} {2}\n", self.r, self.g, self.b));
            return hello;
        }
    }

    impl Add<Color> for Color {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {r: self.r + other.r, g: self.g + other.g, b: self.b + other.b}
        }
    }


    impl Sub<Color> for Color {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self {r: self.r - other.r, g: self.g - other.g, b: self.b - other.b}
        }
    }

    // change this implementation to multiply scalar with vector
    impl Mul<Color> for Color{
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            Self {r: self.r * other.r, g: self.g * other.g, b: self.b - other.b}
        }
    }

    impl Neg<Color> for Color {
        type Output = Self;

        fn neg(self) -> Self {
            Self {r: -self.r, g: -self.g, b: -self.b}
        }
    }

    impl Div<Color> for Color {
        type Output = Self;

        fn div(self, other: u32) -> Self {
            if other == 0 {
                panic!("Cannot divide by zero");
            }
            Self {r: self.r / other, g: self.g / other, b: self.b / other}
        }
    }{}
}
