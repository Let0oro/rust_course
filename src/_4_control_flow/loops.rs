pub fn main() {
    loops();
    while_loop();
    recursive_function();
}

fn loops() {
    // Loop
    let mut counter: u8 = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter == 10 {
            break;
        }
    }
}

fn while_loop() {
    let mut counter: u8 = 0;
    while counter < 10 {
        counter += 1;
        println!("Counter: {}", counter);
    }
}

fn recursive_function() {
    fn recursive_function(counter: u8) {
        if counter >= 10 {
            println!("Counter: {}", counter);
            recursive_function(counter + 1);
        }
    }
    recursive_function(0);
}
