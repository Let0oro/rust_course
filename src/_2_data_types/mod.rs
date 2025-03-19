mod casting_types;
mod floats;
mod integers;
mod sizes;
mod strings;
use casting_types as casting;
mod arrays;
mod chars;
mod exercise;
mod ranges;
mod traits;
mod tuples;

pub fn main() {
    integers::main();
    sizes::main();
    strings::main();
    floats::main();
    casting::main();
    chars::main();
    arrays::main();
    traits::main();
    tuples::main();
    ranges::main();
    exercise::main();
}
