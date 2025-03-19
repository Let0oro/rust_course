pub fn main() {
    // Ranges
    // Ranges are a type that represents a sequence/interval of numbers.
    // You can create a range using the .. or ..= operators.
    // The .. operator creates a range that does not include the end value,
    // while the ..= operator creates a range that includes the end value.

    println!("Ranges\n");

    println!("Ranges are a type that represents a sequence/interval of numbers. \n
  // You can create a range using the .. or ..= operators. \n
  // The .. operator creates a range that does not include the end value, while the ..= operator creates a range that includes the end value.");

    let range: std::ops::Range<i32> = 1..5;

    for number in 1..5 {
        // range.clone()
        println!("{}", number);
    }
    println!("{:#?}", &range);

    // Para evitar usar .clone() pero mantener la variable, podemos convertir el rango en un iterador / vector

    let letters: Vec<char> = ('a'..='f').collect();

    for letter in &letters {
        println!("{}", letter);
    }
    println!("{:#?}", &letters);

    let colors: [&str; 6] = ["red", "green", "blue", "yellow", "black", "white"];
    for color in colors {
        println!("{} is a great color!", color);
    }

    println!("---\n");
}
