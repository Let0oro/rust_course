pub fn main() {
    println!(" - Arrays, tuples and strings are colection types");
    println!(
        " - A slice is a reference (not take ownership) to a portion/sequence of a collection type, a subcategory of reference"
    );

    println!("\nExamples:\n");

    let action_hero = String::from("Arnold Schwarzenaegger");
    let action_hero_ref_str = "Arnold Schwarzenaegger";

    let first_name = &action_hero[0..6];
    let first_name = &action_hero_ref_str[0..6];

    println!(
        " - [STR] action_hero: String = String::from(\"Arnold Schwarzenaegger\") | action_hero_ref_str: &str = \"Arnold Schwarzenaegger\""
    );

    println!(
        " - [STR] let first_name = &action_hero[0..6] || &action_hero_ref_str[0..6] -> {first_name}, is type &str, a reference of a binary data on the memory, is not possible to access to the value"
    );

    let len = &action_hero.len();

    let last_name = &action_hero[7..len - 1];
    let last_name = &action_hero_ref_str[7..len - 1];

    println!(
        " - [STR] let last_name = &action_hero[7..] ||  &action_hero_ref_str[7..len-1];  -> {last_name}, is type &str, a reference of a binary data on the memory, is not possible to access to the value"
    );

    let first_name = {
        let action_hero = "Arnold Schwarzenaegger";
        &action_hero[..6]
    };

    println!(
        " - [STR] {first_name} exists even the main reference ends, cause the slice reference is independent, and the main reference is a binary text incrusted into the file, at its memory"
    );

    let example_sl_arr = {
        let main_arr = &[1, 2, 3, 4];
        &main_arr[..2]
    };

    println!(
        " - [ARR] {:?}, in the same case, but with an array, we only access to the reference referencing the main array -> \" let main_arr = &[1, 2, 3, 4]; \" and returning the &main_arr[..] or...",
        example_sl_arr
    );

    let values = [4, 8, 15, 3242, 23];
    let my_slice = &values[0..4];

    println!(
        " - [ARR] \" let values = [4, 8, 15, 3242, 23]; let my_slice = &values[0..4];\": {:?}",
        my_slice
    );

    println!(
        " - while we are slicing a string, we could take care of the indexes, cause the emogis and other characters could be represented by more than one byte, and the slice could be cutted in the middle of the character, resulting in a panic"
    );

    println!(
        " - The &str type can support the String and &String types, but the String type can't support the &str type, cause the &str type is a reference, and the String type is a binary text incrusted into the file, at its memory"
    );

    do_hero_stuff("Arnold Schwarzenaegger", "&str");
    do_hero_stuff(&String::from("Sylvester Stallone"), "&String");

    println!("---\n")
}

fn do_hero_stuff(hero_name: &str, typ: &str) {
    println!(" [STR] {hero_name} ({typ}) saves the day");
}
