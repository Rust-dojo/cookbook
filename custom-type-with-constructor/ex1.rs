use subset_types::PositiveFloat;

struct Circle {
    radius: PositiveFloat,
}

fn area(r: f64) -> f64 {
    3.14159 * r * r
}

//main thread can return a Result
pub fn main() -> Result<(), String> {
    // let radius = PositiveFloat(2.0); // compiling error
    // let radius = PositiveFloat::new(-2.0); // Err() return

    //the ? shortcut early returns in case of Err
    let radius = PositiveFloat::new(2.0)?;
    //shorthand for when a label and a value are the same (punning)
    let c = Circle { radius };
    Ok(println!("area = {}", area(c.radius.to_float())))
}

mod subset_types {
    // newtype pattern inside a module, the tuple element is private by default
    pub struct PositiveFloat(f64);

    impl PositiveFloat {
        //constructor that can return an error string
        pub fn new(value: f64) -> Result<Self, String> {
            match value {
                p if p >= 0.0 => Ok(Self(p)),
                _ => Err("a positive number was expected".to_string()),
            }
        }
        //public method to get the private f64 tuple value
        pub fn to_float(&self) -> f64 {
            self.0
        }
    }
}

