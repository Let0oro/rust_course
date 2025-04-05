

pub fn main() {

    println!(" - Option Enum models a scenario where a type could be a valid value or nothing at all.");
    println!("   -> This solve the problem of ideas with missing values.");
    println!("   -> 'null' or 'undefined' in js, 'None' on python.");
    println!("   -> Represents the absence of a value.");
    println!("   -> Representation:");
    println!("      Option::None (represents an absent value)");
    println!("      Option::Some(T) (represents a present value, tuple as associated value)");

    let a = Option::Some(5); // -> Option<i32>
    let b = Option::Some("hello"); // -> Option<&str>
    let c = Option::Some(true); // -> Option<bool>

    let a: Option<i8> =  Option::Some(5); // -> Option<i8>
    let a = Option::<i16>::Some(5); // -> Option<i16>

    let d = Option::None::<&str>;
    let d = None::<&str>;
    let d: Option<&str> = None;
    // Are the same

    println!("\nBasic examples of Option Enum:

    let a = Option::Some(5); -> Option<i32>
    let b = Option::Some(\"hello\"); -> Option<&str> (b: {b:?})
    let c = Option::Some(true); -> Option<bool> (c: {c:?})

    let a: Option<i8> =  Option::Some(5); -> Option<i8>
    let a = Option::<i16>::Some(5); -> Option<i16>
    a: {a:?}

    let d = Option::None::<&str>;
    let d = None::<&str>;
    let d: Option<&str> = None;
    -> d === d === d
    d: {d:?}
    ");

}