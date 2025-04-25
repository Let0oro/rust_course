//! This is a documentation comment that serves to teach about all the file
/*!
This is the same but multiline
*/

/// This is a documentation comment which serves as documentation to future developers
/**
This is another documentation comment but with more lines and simplicity, both two serves to teach about the element in the following line
*/


mod inventory;
mod orders;
mod exercise;

use colored::Colorize;


pub fn main () {
    println!("Warehouse and project structure (packages and crates):\n");

    println!(" - Cargo new -> create a rust package (Cargo.toml -> metadata about package name and version)\n");
    println!(" - A package is a collection of one or more crates");
    println!(" - A crate is a collection of Rust code that produces an executable library");
    println!(" - A crate is the smallest amount of code that the Rust compiler considers at a time");
    println!(" - There are two categories of crates:");
    println!("   -> A binary crate is a crate tha compiles to an executable (has a main function that is the entrypoints for the executable)");
    println!("   -> A library crate exports functionality for other Rust programs to share and use (not have a main fn and not compile to be an executable program)\n");

    println!(" - The cargo new command will default to creating a package with a binary crate");
    println!(" - The Cargo.toml file's name sets the name of the package");
    println!(" - Cargo will look for a {} file. If it exists, Rust infers that we have a {} crate named like the name in .toml", "src/main.rs".bold(), "binary".bold());
    println!(" - Cargo also will look for a {} file. If it exists, Rust infers that we have a {} crate named like name in .toml", "src/lib.rs".bold(), "library".bold());

    println!("\n--\n");

    println!("A module is an organizational container that encapsulates related code");
    println!("We can declare a module with the 'mod' keyword followed to the namespace in snake_case");
    println!("Example:

    mod inventory {{
        const FLOOR_SPACE: i23 = 10000;
        const MANAGER: &str = Ivan Inventory;

        #[derive(Debug)]
        enum ProductCategory {{
               Ladder,
               Hammer,
        }}

        #[derive(Debug)]
        struct Item {{
            name: String,
            category: ProductCategory,
            quantity: u32,
        }}

        fn take_to_manager () -> Item {{
            println!('Hey, {{MANAGER}}, hows your coffee?');
        }}

    }}

    ");


    println!("\n--\n");

    println!("The 'pub' keyword is necessary to share any element out of their modules, cause the elements are private by default");

    println!("The manager of our project is {}", inventory::MANAGER);

    println!("\n--\n");

    println!("The benefit of namespaces:");
    println!("Another advantage of modules is that the namespaces can be the same name in other other modules, without shadowing, its not necessary to unrepeat the names");

    println!("\n--\n");
    println!("Modules and submodules");

    println!("A crate root is the base/foundation of a crate (the starting point for the compiler) -> main.rs in the main root");

    println!("My orders manager: {}", orders::MANAGER);

    println!("Path to orders: ./orders/mod.rs");
    println!("Path to inventory: ./inventory.rs");

    println!("We cant have a module folder type and a module dile type with the same name at same time");

    println!("The submodule says: {}", orders::submodule::GREET);
    println!("the route to this is: ./orders/submodule/mod.rs, and i had needed to make public the module in orders/mod.rs");

    println!("\n--\n");

    println!("The 'crate' key:");

    println!("To place us at the path level of crate root, we can use 'crate', from it we can use the 'absolute path', else, we can use the 'relative path' from this location");

    println!("this is an absolute path example 'crate::_13_warehouse::inventory::FLOOR_SPACE': {}", crate::_13_warehouse::inventory::FLOOR_SPACE);

    println!("If we want to access to more namespaces, or simplify the mod namespace, we can use the 'use' key, that point to a more specific namespace, for example, the same as one top mod would be like:
    'use crate::_13_warehouse::::inventory::{{FLOOR_SPACE, MANAGER}}', and we can use it simply for his last namespace 'MANAGER'
    But we can also assign this to an alias, like 'use ...::MANAGER as manager_boss', and call it by 'manager_boss', and we also can make it public with 'pub use ... as ,,,' or 'pub use ...::{{specific_element}}"
    );

    println!("\n--\n");

    println!("The 'self' keyword:");

    let my_value = 10;

    println!("self is a reference to the same module where its called, for example, if i want to bring up the 'products' module and the 'ProductCategory' enum, we can use:
    'use inventory::products::{{self, ProductCategory}}' and calling the 'products' module simply 'products', not by 'self'
    ",);

    println!("\n--\n");

    println!("The 'super' keyword:");

    println!("To navigate to other superior module (parent module), we can use the 'super' keyword");

    println!("\n--\n");

    println!("External crates");
    println!("A dependency is an external library crate that we pull into our project. Our code depends on it to run");

    println!("It it showed into Cargo.toml file, at the '[dependencies]' section");
    println!("We can use 'cargo add <dependency_name>' to imports a new external crate and its functionality");
    println!("Example of the result: 'fake = \"2.9.2\"' or 'fake = {{version: \"2.9.2\", features: [\"derive\"]}}' -> features aid to get only the necessary functionality");
    println!("To use the dependency in our files, we can type 'use fake::Dummy'");
    println!("* fake dependency depends on random dependencies and make fake date to our elements to prove them, like:

    use fake::{{Dummy, Fake, Faker}}

    #[derive(Debug, Dummy)]
    pub struct Item {{
        pub name: String;
        pub category: ProductCategory,
        pub quantity: u32,
    }}

    fn main {{
        let fake_item: Item = Faker.fake(); -> only providing the type, this will fill the namespaces of the Item struct, and this with all other cases or constructions, like Enums or i32, etc.
    }}
    ");

    println!("** The conventional structure of a file importations is:

    //1.
    mod ...

    //2.
    use ... (Rust standard library)

    //3.
    use ... (External crates)

    //4.
    use ... (Local importations)
    ");

    println!("\n--\n");

    println!("The standard library:");
    println!("Is a collection of modules built into Rust, we can access it by use 'std::...', every function we had used without import it from external crates are owned of the standard library");

    println!("\n--\n");

    println!("The Glob operator '*': in module importations, bring in all public elements in their crate, its not recommended cause may cause confusion");

    println!("\n--\n");

    println!("Library crate:");

    println!(" - In opposition to the binary crate (used until now), this will not be compiled");
    println!(" - The way to create this is simply make a new 'lib.rs' file, so: main.rs = binary; lib.rs = library");
    println!(" - It has been designed like a module we can share/export to be used by other programmers (will be automatically shared in crates.io)");
    println!(" - A module need to have a main.rs or a lib.rs or both");
    println!(" - We can use the lib.js crate like an external library with functionality");
    println!("For example (imagine that all our project until now is the 13th section, causing our cargo.toml has the name '_13_warehouse'): ");
    println!("//main.rs:

    use fake::{{Fake, Faker}};

    use _13_warehouse::{{Item, ProductCategory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER}}; // Will point to lib.rs

    fn main () {{

        println!(\"Our managers are {{}} and {{}}. We have {{}} square feet of floor space\", INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE);

        let fake_item: Item = Faker.fake();
        println!(\"{{:?}}\", fake_item);

        let random_category: ProductCategory = Faker.fake();
        println!(\"{{:?}}\", random_category);
    }}
    ");

    println!("//lib.rs:

    mod inventory;
    mod orders;

    pub use inventory::{{Item, ProductCategory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER}};
    pub use orders::MANAGER as ORDERS_MANAGER;

    ");

    println!("\n--\n");

    println!("We can use multiple binary crates (main.rs) in our module/project");
    println!("For this, we need to create a new folder in the main root (src/) called 'bin' (binary)");
    println!("Every rust file created in, Rust will trait that like a independent binary crate or project and compile it");
    println!("If we run 'cargo run', the compiler give as an error cause he doesn't known about with of the bin files execute");
    println!("The solution is:");
    println!(" - If we have src/main and src/bin and we want ro run src/main -> 'cargo run --bin _13_warehouse'");
    println!(" - If we have src/main and src/bin and we want ro run src/bin/example -> 'cargo run --bin examplef'");

    println!("\n--\n");

    println!("To make 'documentation comments' we can type '///' and the compiler will generate a webpage with all this documentation by using the 'cargo doc' command");

    println!("If we run the command 'cargo doc --no-deps', the webpage generated with documentation not generate documentation of external crates in dependencies");

    println!("If we add '--open', the html files will be automatically opened in the browser");

    println!("I LOVE IT!!!");

    println!("For example, to assign a documentation to an element we should type it one line up it and ///, and for the file, //! at top of the file or module");

    println!("\n---\n");
}