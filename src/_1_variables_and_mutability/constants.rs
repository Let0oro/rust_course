type BigNum = f64; // This is a type alias

const TAX_RATE: BigNum = 21.0;

pub fn main() {
    let mut income: String = String::new();
    println!("Enter your income: ");
    std::io::stdin()
        .read_line(&mut income)
        .expect("Failed to read line");

    let income: BigNum = income.trim().parse().unwrap();
    let tax = income * TAX_RATE;
    let final_tax = income * (100.0 + TAX_RATE) / 100.0;

    print!(
        "The tax on ${} is ${}, causing final price of {}",
        income, tax, final_tax
    );
}
