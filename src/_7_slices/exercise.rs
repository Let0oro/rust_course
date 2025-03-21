pub fn main() {
    println!("Exercise:\n");

    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two = &cereals[..2];
    println!("first two: {:?}", first_two);

    let mid_three = &cereals[1..4];
    println!("first two: {:?}", mid_three);

    let last_three = &mut cereals[2..];
    println!("first two: {:?}", last_three);

    last_three[4] =  String::from("Lucky Charms");

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[..5];
    println!("cookie: {}", cookie);


    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[5..];
    println!("puffs: {}", puffs);


    println!("---\n");
}
