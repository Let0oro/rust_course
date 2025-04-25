use std::fmt::{Debug, Display, Formatter, Result};
use std::fs;
use std::ops::Drop;
use std::clone::Clone;

enum AppleType {
    RedDelicious,
    GrannySmith
}

impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith 🍏"),
        }
    }
}

/**
May be customized like Display:
```
fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
    match self {
        AppleType::RedDelicious => write!(formatter, "AppleType::Delicious 🍎"),
        AppleType::GrannySmith => write!(formatter, "AppleType::Granny Smith 🍏"),
    }
}
```
or commonly on Debug
```
fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_tuple("AppleType").field(self).finish()
}
```
*/
impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "AppleType::Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "AppleType::Granny Smith 🍏"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64
}

/// formatter -> the place where we will write on
/// write! -> macro for write something in somewhere (formatter)
impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}🍏 for {}€", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.debug_struct("** Apple **")
        .field("Kind", &self.kind)
        .field("Price", &self.price)
        .finish()
    }
}

/// The Drop trait has a drop method which is called when the memory are been realocated
/// Imagine that an apple.txt is created every time an Apple struct is created, if we drop the struct, not necessarily will remove the .txt, then we will use this
impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("apple.txt") {
            Ok(_) => println!("Goodbye, my sweet apple"),
            Err(error) => eprintln!("Error deleting file: {error}")
        };
    }
}

struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string()
        }
    }
}

/// The Clone trait models the ability to create a duplicate of an instance \
/// This is equal to the default, so if we comment this block and make a ```#[derive(Clone)]``` up to Appointment, we will get teh same result
impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone()
        }
    }
}

#[derive(Debug, Clone)]
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self { hours, minutes, seconds }
    }
}

/// The Copy trait is a subtrait of Clone, so when we derive the Duration struct, we not need to do more
impl Copy for Duration {}

#[derive(Debug)]
struct BusTrip {
    origin: String,
    destination: String,
    time: String
}

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    time: String
}

/// The PartialEq trait establishes equality between two values.
impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string()
        }
    }
}

impl BusTrip {
    #[allow(clippy::new_ret_no_self)]
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string()
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        let same_origin = self.origin == other.origin;
        let same_destination = self.destination == other.destination;
        same_origin && same_destination
    }

    fn ne(&self, other: &Self) -> bool {
        let same_origin = self.origin == other.origin;
        let same_destination = self.destination == other.destination;
        !same_origin || !same_destination
    }
}

/// PartialEq has a default value Self as Rhs ('right side hand') with is used to compare, but we can define other
impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        let same_time = self.time == other.time;
        same_time
    }
}

pub fn main() {
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04
    };

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.15
    };

    println!("{:?}", lunch_snack);
    println!("{:?}", dinner_snack);

    let morning_appt = Appointment::new("Dr. Andrews", "9:00AM", "10:00AM");
    let replacement_appt = morning_appt.clone();
    println!("{} is seeing the patient form {} to {}",
             replacement_appt.doctor, replacement_appt.start_time, replacement_appt.end_time
    );

    let one_hour = Duration::new(1, 0, 0);
    let another_hour = one_hour;

    println!("{:?}", one_hour);

    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:20");
    let c = Flight::new("New York", "Los Ángeles", "08:00");
    let d = BusTrip::new("Los Angeles", "Tokio", "08:00");

    println!("{a:?} == {b:?} -> {}", a.eq(&b));
    println!("{a:?} == {c:?} -> {}", a.eq(&c));
    println!("{b:?} == {c:?} -> {}", b.eq(&c));
    println!("{a:?} != {b:?} -> {}", a.ne(&b));
    println!("{a:?} != {c:?} -> {}", a.ne(&c));
    println!("{b:?} != {c:?} -> {}", b.ne(&c));
    println!("Flight {c:?} time == BusTrip {d:?} time -> {}", c.eq(&d));
    //c == d -> OK // d == c -> BAD, cause '==' is using backwards '.eq', and the BusTrip not has the PartialEq trait with Flights
    // SO: THE OPERATORS INVOKE METHODS BEHIND THE SCENES
    println!("Flight {c:?} time == BusTrip {d:?} time -> {}", c == d);
    // println!("Flight {c:?} time == BusTrip {d:?} time -> {}", d == c); -> Error
}