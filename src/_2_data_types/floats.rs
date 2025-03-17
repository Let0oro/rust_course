

pub fn main() {
  println!("Floats\n");

  let pi: f64 = 3.14159265358979323846264338327950288419716939937510;

  println!("pi: {}", pi);
  println!("pi with 2 decimals (:.2): {:.2}", pi);
  println!("pi.floor(): {}", pi.floor());
  println!("pi.ceil(): {}", pi.ceil());
  println!("pi.round(): {}", pi.round());
  println!("pi.trunc(): {}", pi.trunc());
  println!("pi.fract(): {}", pi.fract());
  println!("pi.sqrt(): {}", pi.sqrt());
  println!("pi.powi(2): {}", pi.powi(2));

  println!("Base 10:               {}", 251);
  println!("Binary      (base 2, :b):  {:b}", 251);
  println!("Octal       (base 8, :o):  {:o}", 251);
  println!("Hexadecimal (base 16, :x): {:x}", 251);

  println!("---\n")
}