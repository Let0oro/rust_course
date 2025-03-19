pub fn main() {
    println!("Tuples\n");
    println!("Tuples are a way to group multiple values into a single compound value.");
    println!("The values within a tuple can have different types.");

    let employee: (&str, &str, i32, f64, bool) = ("John", "Doe", 30, 1.80, true);

    dbg!(employee);
    print!(
        " -> (&str, &str, i32, f64, bool).\nEmployee: {} {} is {} years old, is {}m tall and is active: {}\n",
        employee.0, employee.1, employee.2, employee.3, employee.4
    );

    let (name, surname, age, height, active) = employee;
    println!("let (name, surname, age, height, active) = employee;");
    println!(
        "Destructuring the tuple.\nEmployee: {} {} is {} years old, is {}m tall and is active: {}\n",
        name, surname, age, height, active
    );

    println!("{:#?}", employee);

    println!("---\n")
}
