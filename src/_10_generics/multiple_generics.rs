pub fn main() {
    println!("Multiple Generics:\n");

    println!(
        "For example: 'fn make_tuple<T>(first: T, second: i32) -> (T, i32) {{first, second}}'"
    );
    println!(" -> 'make_tuple::<&str>(\"hello\", 5);'\n");

    println!(
        "For example: 'fn make_same_tuple<T>(first: T, second: T) -> (T, T) {{first, second}}'"
    );
    println!(" -> 'make_same_tuple::<&str>(\"hello\", \"world!\");'\n");

    println!(
        "For example: 'fn make_dif_tuple<T>(first: T, second: U) -> (T, U) {{first, second}}'"
    );
    println!(" -> 'make_dif_tuple::<&str, i32>(\"hello\", 5);'\n");

    make_tuple::<&str>("hello", 5);
    make_same_tuple::<&str>("hello", "world!");
    make_dif_tuple::<&str, i32>("hello", 5);

    println!("---\n");
}

fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    (first, second)
}

fn make_same_tuple<T>(first: T, second: T) -> (T, T) {
    (first, second)
}

fn make_dif_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
