pub fn main() {
    println!("Turbofish operator '::<type>'\n");

    println!("Used when we specify the real type passed as argument to a function\n");
    println!("For example: having the previous identify fn, we can specify the argument type:");
    println!("'identify::<i32>(5)'\n");

    println!("We can customize the turbofish operator if we wat a different type\n");
    println!("'identify::<u32>(5)'\n");
    println!("'identify::<i8>(5)'\n");
    println!("'identify::<f32>(13.12)'\n");
    println!("'identify::<String>(String::from(\"hello\"))'\n");
    println!("'identify::<bool>(true)'\n");
    println!("'identify::<DeliSandwich>(DeliSandwich {{}})'\n");

    println!("---\n");
}

fn identify<T>(value: T) -> T {
    value
}
