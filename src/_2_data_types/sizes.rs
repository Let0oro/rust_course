pub fn main() {
    println!("Sizes\n");

    println!(
        "The size of a type in memory is the same in all operating systems, but the size of the memory can change depending on the operating system.\n"
    );
    println!(
        "The usize / isize type is the same size as the pointer size of the computer's architecture.\n"
    );
    println!(
        "If my computer uses a 64-bit architecture, the usize / isize type will be 64 bits, and if it uses a 32-bit architecture, the usize / isize type will be 32 bits.\n"
    );

    let size_of_i8: usize = std::mem::size_of::<i8>();
    let size_of_u8: usize = std::mem::size_of::<u8>();

    let size_of_i16: usize = std::mem::size_of::<i16>();
    let size_of_u16: usize = std::mem::size_of::<u16>();

    let size_of_i32: usize = std::mem::size_of::<i32>();
    let size_of_u32: usize = std::mem::size_of::<u32>();

    let size_of_i64: usize = std::mem::size_of::<i64>();
    let size_of_u64: usize = std::mem::size_of::<u64>();

    let size_of_i128: usize = std::mem::size_of::<i128>();
    let size_of_u128: usize = std::mem::size_of::<i128>();

    println!("i8: {} bytes", size_of_i8);
    println!("u8: {} bytes", size_of_u8);

    println!("i16: {} bytes", size_of_i16);
    println!("u16: {} bytes", size_of_u16);

    println!("i32: {} bytes", size_of_i32);
    println!("u32: {} bytes", size_of_u32);

    println!("i64: {} bytes", size_of_i64);
    println!("u64: {} bytes", size_of_u64);

    println!("i128: {} bytes", size_of_i128);
    println!("u128: {} bytes", size_of_u128);

    println!("---\n")
}
