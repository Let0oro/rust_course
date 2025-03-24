#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn show_string(&self) {
        println!("{:?}", self);
    }
}

impl TreasureChest<[&str; 3]> {
    fn show_string_three_array(&self) {
        println!("{:?}", self);
    }
}

impl<T: std::fmt::Debug> TreasureChest<T> {
    fn show_generic_type(&self) {
        println!("{:?}", self);
    }
}

pub fn main() {
    println!("Structs generics:\n");

    println!("Declaration:

    struct TreasureChest<T> {{
        captain: String,
        treasure: T,
    }}

    Invocation:
    let gold_chest: TreasureChest<i32> = TreasureChest::<i32>...
    let silver_chest: TreasureChest<String> = TreasureChest...
    let special_chest: TreasureChest<[&str, 3]> = TreasureChest...

    ");

    let gold_chest: TreasureChest<i32> = TreasureChest::<i32> {
        captain: String::from("Garfio"),
        treasure: 4,
    };

    println!("gold_chest -> {:?}\n", gold_chest);

    let silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("Firebeard"),
        treasure: String::from("Silver")
    };

    println!("silver_chest -> {:?}\n", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"]
    };

    println!("special_chest -> {:?}\n\n", special_chest);

    println!("Generics and Impl Blocks:\n");

    println!("We can define impl methods to a struct with generic types, but the sintaxis changes:

     - Specified: impl TreasureChest<type> {{}}, like impl TreasureChest<String> {{}}.
       -> This will run only for the String (or chosen type), meaning we can chose the behavior of the impl TreasureChest in concrete cases.
       -> For example, we ONLY can use this methods or constructors in String type TreasureChest created structs, like 'silver_chest'.
       -> OK silver_chest.show_string();
       -> NO gold_chest.show_string();
       -> Other examples:
          · NO impl TreasureChest<[&str]>
          · OK impl TreasureChest<[&str; 3]>

     - Generic:
       impl<T> TreasureChest<T> {{
          fn show_generic_type(&self) {{
              println!(\"{{:?}}\", self);
          }}
       }}
       -> This resolve the problem with the undefined 'T' type if we declare it after 'impl TreasureChest<T>', cause this 'T' is only declared in the struct block,
       -> The 'impl<T>' declare the generic type before, and allows to run other defined types (String, &str) as concrete type of impl.
       -> Examples:
          · OK gold_chest.show_generic_type();
          · OK silver_chest.show_generic_type();
          · OK special_chest.show_generic_type();

        * impl<T: std::fmt::Debug> allow to print ind debug format without causing errors

    ");


    println!("---\n");
}
