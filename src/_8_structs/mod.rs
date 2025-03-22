mod in_fn;
mod asociated_fn;
mod build_pattern;
mod tuple_structs;
mod unit_structs;
mod exercise;

pub fn main() {
    println!("Structs:");

    println!(" - A struct (structure) is a  container for related pieces of data");
    println!(" - Is comparable to an object nd similar to a class");
    println!(
        " - We use structs to represent complex data types that can store multiple pieces of information"
    );

    println!(" - Rust has 3 kind: ");
    println!("    -> Named Field Structs");
    println!("    -> Tuple-Like Structs");
    println!("    -> Unit-Like Structs\n");
    println!(" - Are typyfied in CamelCase\n");
    println!(" - A struct describe a project typified\n");

    let coffee = ("Caramel Macchiato", 5.99, true);

    println!(
        " - A struct will be the owner of a field or a name, and this name or field will be the owner of its own value\n"
    );

    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    println!(
        "  struct Coffee {}
    price: f64,
    name: String,
    is_hot: bool,
  {}
  ",
        "{", "}"
    );

    let mocha = Coffee {
        price: 4.99,
        name: String::from("Mocha"),
        is_hot: true,
    };

    println!(
        "Mocha: 
  name = {} (mocha.name), 
  price = {} (mocha.price), 
  is_hot = {} (mocha.is_hot)",
        mocha.name, mocha.price, mocha.is_hot
    );

    let favorite_coffee = mocha.name;
    println!("let favorite_coffee = mocha.name; -> The ownership has changed to favorite_coffee from the struct mocha.name");


    
    let mut beverage = Coffee {
      price: 4.99,
      name: String::from("Mocha"),
      is_hot: true,
  };

  beverage.name = String::from("Caramel Macchiato");
  beverage.price = 5.99;
  beverage.is_hot = false;




  in_fn::main();
  asociated_fn::main();
  build_pattern::main();
  tuple_structs::main();
  unit_structs::main();
  exercise::main();  

    println!("---\n");
}
