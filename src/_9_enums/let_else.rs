enum Milk {
    Lowfat(i32),
    Whole,
    NonDiary { kind: String },
}

pub fn main() {
    println!("Let else with enums (commonly used):\n");

    let my_beverage = Milk::Lowfat(3);
    println!(
        "For example, if the comparison succeed, this means the left-hand enum variant is the same that right-hand, and the value of right-hand is the value of the name typed on the left-hand enum variant:"
    );
    println!(" -> let Milk::Lowfat(percentage) = Milk::Lowfat(100); -> 'let percentage = 100'");

    let Milk::Lowfat(percentage) = my_beverage else {
        println!(
            "This print line not succeed if the comparison up is true, then we need to return cause we are declaring a value is not defined."
        );
        return;
    };

    println!("---\n");
}
