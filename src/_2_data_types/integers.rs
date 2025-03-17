
pub fn main () {
  println!("Integers\n");

  let eight_bit_signed: i8 = -112; // i8
  let eight_bit_unsigned: u8 = 112; // i8

  let sixteen_bit_signed: i16 = -32500;
  let sixteen_bit_unsigned: u16 = 64500;

  let thirty_two_bit_signed: i32 = -2147483648; // i32
  let thirty_two_bit_unsigned: u32 = 4290000000; // i32

  let sixty_four_bit_signed: i64 = -9_223_372_036_854_775_808; // i64
  let sixty_four_bit_unsigned: u64 = 18_446_744_073_709_551_615; // i64

  let one_hundred_twenty_eight_bit_signed: i128 = -170141183460469231731687303715884105728; // i128
  let one_hundred_twenty_eight_bit_unsigned: u128 = 340282366920938463463374607431768211455; // i128

  let some_value: i8 = 20i8; // Declaring the type of the variable inside the value


  println!("i8: {} -> 8 bites = 1byte of memory", eight_bit_signed.to_string());
  println!("u8: {}", eight_bit_unsigned.to_string());
  println!("i16: {} ->16 bites = 2bytes of memory", sixteen_bit_signed.to_string());
  println!("u16: {}", sixteen_bit_unsigned.to_string());
  println!("i32: {} -> 32 bites = 4bytes of memory", thirty_two_bit_signed.to_string());
  println!("u32: {}", thirty_two_bit_unsigned.to_string());
  println!("i64: {} -> 64 bites = 8bytes of memory", sixty_four_bit_signed.to_string());
  println!("u64: {}", sixty_four_bit_unsigned.to_string());
  println!("i128: {} -> 128 bites = 16bytes of memory", one_hundred_twenty_eight_bit_signed.to_string());
  println!("u128: {}", one_hundred_twenty_eight_bit_unsigned.to_string());
  println!("I can declare the type of an integer only writing right next the type, like this i8: {}i8", some_value.to_string());
  println!("I can use '_' to separate numbers: -9_223_372_036_854_775_808 = {} or  18_446_744_073_709_551_615 = {}", sixty_four_bit_signed.to_string(), sixty_four_bit_unsigned.to_string());

  println!("---\n")
}