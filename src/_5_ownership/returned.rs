pub fn main() {
    println!("Returned:\n");
    let cake: String = bake_cake();
    println!("Cake: {}", cake);

    println!("---\n");
}

fn bake_cake() -> String {
    String::from("Chocolate mousse")
}
