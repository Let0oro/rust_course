
#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special (&self) -> Option<Food> {

        if self.has_mice_infestation { None } else {
            match self.reservations {
                people if people < 12 => Some(Food { name: "Uni Sashimi".to_string() }),
                _ => Some(Food { name: "Strip Steak".to_string() }),
            }
        }
    }

    fn deliver_burger (&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            Err("Sorry, we have a mice problem".to_string())
        } else {
            if address.is_empty() {
                Err("No delivery address specified".to_string())
            } else {
                Ok(Food {name: "Burger".to_string()})
            }
        }
    }
}


pub fn main() {

    let bad_restaurant = Restaurant {reservations: 11, has_mice_infestation: true};

    println!("BR Example of chef special (11 res, has_mice: true): {:?}",
    bad_restaurant.chef_special()
    );

    println!("BR Example of delivery burger (not empty address): {:?}",
    bad_restaurant.deliver_burger("123 Elm Street")
    );

    let good_restaurant = Restaurant {reservations: 15, has_mice_infestation: false};

    println!("GR Example of chef special (15 res, has_mice: false): {:?}",
    good_restaurant.chef_special()
    );

    println!("GR Example of delivery burger (empty address): {:?}",
    good_restaurant.deliver_burger("")
    );

    println!("GR Example of delivery burger (not empty address): {:?}",
    good_restaurant.deliver_burger("123 Elm Street")
    );
}