enum Milk {
    Lowfat(i32),
    Whole,
    NonDiary { kind: String },
}

pub fn main() {
    println!("If let with enums (commonly used):\n");

    let my_beverage = Milk::Lowfat(3);

    if let Milk::Lowfat(percentage) = my_beverage {
        println!(
            "This print line appears if the comparison up is true, the percentage value is {}%, and i can use only in this block.",
            percentage
        );
    }

    println!("---\n");
}
