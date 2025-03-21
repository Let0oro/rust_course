pub fn main() {
    println!("\nDeref coertion with arrays");
    let values = [4, 8, 15, 16, 32, 43];

    let reg_ref = &values;

    let slice_of_three = &values[..3];

    println!(
        " - The &[<type>] (generic/dynamic array number) type can support the &[<number>; <type>] array type, but the &[<number>; <type>] type can't support the &[<type>] type."
    );

    print_length(reg_ref);
    print_length(slice_of_three);

    println!("---\n");
}

fn print_length(refer: &[i32]) {
    println!("{}", refer.len());
}
