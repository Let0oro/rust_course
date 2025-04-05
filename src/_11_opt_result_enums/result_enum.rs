

pub fn main() {

    println!("The Result Enum");

    println!(" - While the Option enum serves to represent the 'presence' in opposition to the 'absence', the Result enum represent the 'success' in opposition to the 'error'");
    println!(" - 'pub enum Result<T, E> {{ Ok(T), Err(E) }}'");
    println!("                    ^^^^ -> cause there are TWO generic types, we MUST include also TWO in the type declaration, For Example:
                'let ok: Result<i8, &str> = Result::Ok(5)'
                'let disaster: Result<i32, &str> = Result::Err(\"Something went wrong\")'
    ");
    println!(" - The Ok variant indicates a success. It stores an associated piece of data of generic type T.");
    println!(" - The Err variant indicates an error. It stores an associated piece of data of generic type E.");
    println!(" - The same idea in other languages is the exception or the Error type (like the catch parameter in js or its ErrorInstance)\n");

    let ok: Result<i8, &str> = Result::Ok(5);
    let disaster: Result<i32, &str> = Result::Err("Something went wrong");

    println!("{ok:?}");
    println!("{disaster:?}\n");

    println!("---\n");
}