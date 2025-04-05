pub fn main() {
    println!("Arrays:\n");
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];

    let first = numbers[0];
    let _second = numbers[1];

    println!("The first number is: {}", first);

    let apples: [&str; 5] = [
        "Granny Smith",
        "Golden Delicious",
        "Fuji",
        "Honeycrisp",
        "Gala",
    ];

    println!("Length of apples: {}", apples.len());

    let _empty_array: [f64; 0] = [];

    println!("\nReading and writing elements of an array\n");
    let mut seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
    println!("The seasons are: {:?}", seasons);

    seasons[2] = "Autumn";

    println!("The seasons are: {:?}", seasons);

    println!("---\n")
}
