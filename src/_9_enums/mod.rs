#![allow(dead_code, unused_assignments)]

mod associated_vals_enum;
mod exercise;
mod if_let;
mod let_else;
mod match_enum;
mod match_enum_2;
mod match_enum_3;
mod match_enum_4;
mod nesting_enum;

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit,
}

pub fn main() {
    println!("Enums:\n");

    println!(
        " - An Enum is a type that represents a set of possible values. Each possible values is called a 'variant'"
    );
    println!(" - Both enum name and variants name are typed on PascalCase way");

    println!(
        "\nExample:
    enum CardSuit {{
      Hearts, 
      Diamonds,
      Spades,
      Clubs,
    }}
  "
    );

    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;

    println!("{:#?}, imply an idea of the type would be.", second_card);

    let card_suit = [CardSuit::Hearts, CardSuit::Clubs];
    let card_suit = (CardSuit::Spades, CardSuit::Diamonds);

    associated_vals_enum::main();
    nesting_enum::main();
    match_enum::main();
    match_enum_2::main();
    match_enum_3::main();
    match_enum_4::main();
    if_let::main();
    let_else::main();
    exercise::main();

    println!("---\n");
}
