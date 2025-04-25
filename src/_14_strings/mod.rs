use std::io;
use std::ops::Add;

pub fn main() {
    println!("Strings:");

    println!(" - A string is a piece of text or, alternatively, a sequence of characters");
    println!("&str y String");

    println!("To access to the very first character, we shouldn't use the [0] syntax, else the [0..1] range syntax will run better, cause we are pointing to a bite range no a character range");
    println!("String::from(\"Bloodhook\")[0] BAD");
    println!("String::from(\"Bloodhook\")[0..1] GOOD");
    println!("String::from(\"Bloodhook\").get(0..1).unwrap() VERY GOOD");

    // println!("{}", String::from("Boodhook")[0]); BAD
    // println!("{}", String::from("Boodhook")[0..1]); BAD, unknown size
    println!("{}", String::from("Boodhook").get(0..1).unwrap());

    println!("\n--\n");

    println!("Concatenation:");

    println!("mut String + .push_str(&str | &String) -> {:#?} -> &str", String::from("Sylvester").push_str("Stallone"));
    println!("mut String + .push('char') -> {:#?}", String::from("Sylvester").push('.'));
    println!("mut String + &str -> {:#?} -> &str", String::from("Sylvester") + "Stallone");
    println!("mut String + &String -> {:#?} -> &str", String::from("Sylvester") + &String::from("Stallone"));
    println!("mut String + .add(&String | &str) [.add method implements .push_str into it]-> {:#?} -> &str", String::from("Sylvester").add("Stallone"));

    println!("\n--\n");

    println!("Macro format (format!(\"...\"))");

    println!("Is the same as println! macro but we can store it in a variable and not print it in console");

    let first_name = String::from("Sylvester");
    let last_name = String::from("Staone");

    let icon = format!("{} {last_name}", first_name);
    println!("{}", icon);

    println!("\n--\n");

    println!("Common string methods");

    let music_genres = "    Rock, Metal, Country, Rap    ";

    println!("{}", music_genres.trim());
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());
    println!("{}", format!("\r {music_genres} \n   ").trim_ascii());
    println!("It always returns \"Rock, Metal, Country, Rap\"");

    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("a", "@"));

    println!("{:#?}", music_genres.split(", ").collect::<Vec<&str>>());

    println!("&str.split() -> Split enum");
    println!("&str.split().collect::<type>() -> Vec or array of a specific type");
    println!("&str.split().collect() -> error");

    println!("\n--\n");

    println!("Collecting user input with read_line");

    let mut name = String::new();

    println!("What's your name");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to collect input from the user");

    println!("Hello {name}");

    let mut age: u32 = 0;
    println!("What's your age");

    match io::stdin().read_line(&mut age.to_string()) {
        Ok(age) => println!("You are {age} years old"),
        Error=> println!("Error collecting age")
    };

    println!("\n---\n");
}