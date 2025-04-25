//! Initially, we will understand the time of life of a variable
//! as the time from declaration of a variable until the end of this scope
//!
//! The borrow checker is the part of the Rust compiler that validates that
//! all borrows (i.e: references) are valid
//!
//! Lexical lifetimes means lasting until the end of the block
//!
//! Non-lexical lifetimes means not lasting until the end of the block
//!
//! If the borrow checker treats the end of a reference's lifetime as the last
//! place it is used, instead of the end of the block; a reference has non-lexical scope.
//!
//!
//! A dangling reference is a reference to a value that no longer exists.
//!
//! We just do operations with references respecting the lifetime of each reference (one at a time).
//!
//! *** &[T] -> its ok for a vector of T elements and a array of T elements
//! *** &[T; N] -> its ok for a array of T elements with N elements
//!
//!
//! The return reference of a function is the same as the reference of the parameter
//!
//!

pub fn main() {
    // Concrete values lifetime
    {
        let c = 1;
        // c lives
    }
    // c no lives more

    let a = String::from("winter");
    // a lives
    drop(a); // or let b = a; \
    // a no lives more

    // ---

    // References lifetime

    let a = String::from("winter");
    let b = &a;
    drop(a);
    // no lives either a nor b

    // Functions with reference lifetimes

    let a = String::from("winter");
    let b = refer(&a);
    drop(a);
    // no lives either a nor b

    // Example of non-lexical lifetime: (the lifetime and the scope are not technically the same)

    let mut data = vec!['a', 'b', 'c']; // <- lifetime (data) starts
    let slice = &mut data[..]; // <- lifetime (slice) starts
    capitalize(slice); // <- lifetime (slice) ends (last time being used)
    data.push('d'); // <-\
    data.push('e'); // <-+- we can operate over data without overwrite its reference (slice)
    data.push('f'); // <-/  lifetime (data) ends

    // Dangling reference:

    let some_cities = {
        let cities = vec![
            String::from("Londres"),
            String::from("New York"),
            String::from("New York"),
        ];
        // &cities[..] error cause cities is dropped here, and the reference will be dangling
    };

    // ---

    // One at a time:

    // ---- cities ----
    let cities = vec![
        String::from("Londres"),
        String::from("New York"),
        String::from("New York"),
    ];

    // --- favorite_cities ---
    let favorite_cities = &cities[..];

    // GOOD: --- end favorite_cities ---

    // --- places ---
    let places = cities;
    // end places

    // println!("favorite_cities: {:?}", favorite_cities); -> error, we are no respecting the favorite_cities scope lifetime operations
    // BAD: --- end favorite_cities --- overwrite place operations

    // end cities
}

fn refer<'a>(r: &String) -> &str {
    let a = &r[..];
    a
}

fn capitalize(c_arr: &mut [char]) -> Vec<char> {
    c_arr
        .iter()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>()
}
