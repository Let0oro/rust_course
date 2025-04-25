/** Traits in Enums \
    Eq trait -> subtrait of PartialEq trait but more strict, this implements by default in other element this three principles:\
    - reflexive: a == a. \
    - symmetric: a == b, implies b == a (required by PartialEq as well).\
    - transitive: a == b and b == c implies a == c (required by PartialEq as well)

By applying the Eq trait up to the struct in the derive box, we also say the compiler that it can trust into the Eq principles

There are two separated traits cause may have uncommon exceptions in these principles,
for example the f32 y f64, cause the floats accept the NaN value -> 0.0/0.0
(only when the float represents a NaN is when its cant be compared)
*/

mod partialord;
mod associated_types;
mod exercise;

#[derive(Eq)]
enum Musician {
    SingleSongWriter(String),
    Band(u32)
}

use Musician::{Band, SingleSongWriter};

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        /**
        match self {
            SingleSongWriter(writer) => match other {
                SingleSongWriter(other_writer) => writer == other_writer,
                Band(_) => false
            },
            Band(band_number) => match other {
                Band(other_band_number) => band_number == other_band_number,
                SingleSongWriter(_) => false
            }
        }
        */

        /// Another way to do but more simplified
        match (self, other) {
            (SingleSongWriter(writer), SingleSongWriter(other_writer)) => writer == other_writer,
            (Band(band_people), Band(other_band_people)) => band_people == other_band_people,
            (SingleSongWriter(_), Band(_)) => false,
            (Band(_), SingleSongWriter(_)) => false,
        }
    }
}

pub fn main() {

    let rustin_bieber = SingleSongWriter("Rustin".to_string());
    let rustin_timbelake = SingleSongWriter("Rustin".to_string());
    let holly = SingleSongWriter("Holly".to_string());

    let rust_no_one = Band(5);
    let unrustworthly = Band(4);
    let rust_for_vengeance = Band(5);

    println!("{}", rustin_bieber == holly);
    println!("{}", rustin_bieber == rustin_timbelake);
    println!("{}", rustin_bieber == rust_no_one);

    println!("{}", rust_no_one == unrustworthly);
    println!("{}", rust_no_one == rust_for_vengeance);

    println!("=== PartialOrd ===");
    partialord::main();

    println!("===Associated Types===");
    partialord::main();

}