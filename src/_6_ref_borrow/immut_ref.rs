pub fn main() {
    println!("Immutable references\n");

    println!(
        "I can use several immutable REFERENCES cause they will always be target to the same value"
    );

    let car: String = String::from("Red");
    let ref1: &String = &car;
    let ref2 = &car;
    let ref3 = &car;

    println!("car: {car} == ref1 (&car): {ref1} == ref2  (&car): {ref2} == ref3  (&car): {ref3}");

    println!("---\n");
}
