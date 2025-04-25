//! Generic types:
//!
//! - A generic is a placeholder for a future type. \
//! - Generics add flexibility by not hardcoding an exact type into the code. \
//! - Code can use a variety of types in a place of the generic. \
//!
//! Lifetime annotations:\
//! - A lifetime annotation is a name or label for a lifetime.
//!
//! - Lifetime annotations don't change the reference's lifetime.
//! They don't affect the logic in any way.
//!
//! - A lifetime annotation is a piece of metadata that we provide to the
//! borrow checker so that it can validate that references are valid.\
//!
//! Is a mark that we add to define the relationship between the lifetimes of multiple references. \

pub fn main() {
    let cities = vec![
        String::from("Londres"),
        String::from("Paris"),
        String::from("Madrid"),
    ];
    let first_two = select_first_two_elements(&cities);
    println!("First two cities: {:?}", first_two);
}

/// lifetime annotation example:
/// The lifetime of the reference returned by this function is the same as
/// the lifetime of the reference passed to it.
fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}
