//! Is the act of omitting something.
//!
//! Lifetime elision means omitting generic lifetime annotations in situations
//! where the borrorw checker can infer the lifetime relationships automatically
//!
//! Elision rules:
//!
//! 1. The compiler assigns a lifetime to each parameter that is a reference.
//!
//! 2. If there is one reference parameter and the return value is a reference,
//! the borrow chwcker will infer that their lifetimes are related.
//!
//! 3. In a method definition, if there are multiple reference parameters but one of
//! them is **self**, the borrow checker will assume the lifetime of the instances is
//! connected to the lifetime of the return value.
//!
//! 4. If there are multiple reference parameters and the return value is a reference,
//! the compiler needs that we declare de lifetime relationship between there.
//!
//! The 'static lifetime infers to the compiler that a value will exist along all the program

#[allow(unused)]
#[derive(Debug)]
struct TravelPlan<'a> {
    from: &'a str,
    to: &'a str,
    name: String,
    // name: &str // BAD: all the references must be a lifetime implemented
}

pub fn main() {
    {
        let orlando = String::from("Orlando");
        let san_francisco = String::from("San Francisco"); // Good cause the lifetime will be respected
        let result = {
            // let san_francisco = String::from("San Francisco"); // BAD cause this is no live enought
            longest(&orlando, &san_francisco)
        };
    }

    {
        let orlando = String::from("Orlando");
        let result = {
            let san_francisco = String::from("San Francisco");
            print_and_return(&orlando, &san_francisco)
        };
    }
}

fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

fn print_and_return<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    println!("The second is {second}");
    first
}
