

pub fn main () {
  println!("Immutable references\n");

  println!(" - If at least i  had declare a mutable reference, i cant use any other immutable or mutable reference cause the value of the ref is possibly to has been changed");

  println!(" - If i dont use at the same time the mutable and immutable reference, i can use the immutable without risk");

  println!(" - It is cause rust detect the lifetime of it own variables, it is the invalidation  of the references before the socpe end");

  println!("\nExample: ");

  let mut car: String = String::from("Red");
  let  ref1: &mut String = &mut car;
  ref1.push_str(" And Silver");
  let ref2 = &car;

  println!("OK: \n1. declare car \n2. declare ref1 as &car \n3. push a string after ref1\n4. declare ref2 as &car: {ref2}.\n");
  println!("NO: \n1. declare car \n2. declare ref1 as &car \n3. declare ref2 as &car: {ref2} \n4.push a string after ref1\n");

  println!("CAUSE the ref2 is not getting the real value of car reference, cause ref 2 is modifing the reference after.");


  println!(" * While the immutable references has the Copy trait, the mutable NOT cause is impossible to have two or more mutable references at same time: two or more changing references to the 'unsafe same' value.");
  println!("---\n");
}