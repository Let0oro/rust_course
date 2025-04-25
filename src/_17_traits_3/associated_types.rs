use std::ops::Add;

/**
    An associated type is a placeholder for a type that is required within a trait

    Fore example, we will want implements the Add trait to our struct Lunch
*/

#[derive(Debug)]
struct Lunch {
    cost: f64
}

impl Add for Lunch {
    type Output = Lunch;

    fn add(self, rhs: Self) -> Self::Output {
        Self { cost: self.cost + rhs.cost }
    }
}

pub fn main() {

}

/// The Output type needs to know the type returned, and for this we need to pass it the generic
fn add_two_numbers<T>(a: T, b: T) -> T
where T: Add< Output = T>
{ a + b }