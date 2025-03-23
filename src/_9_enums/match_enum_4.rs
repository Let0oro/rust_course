enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk is my favourite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've the lowfat {percent} version!");
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }
}

pub fn main() {
    println!("Match enums IV:\n");

    println!(
        "For a very specific match, we can add an arm at the match with a selection value -> ...match {{ Milk::Lowfat(2) => println!(\"Milk is lowfat!\"),..."
    );

    Milk::Lowfat(2).drink();
    Milk::Whole.drink();
    Milk::Lowfat(34).drink();

    println!("---\n");
}
