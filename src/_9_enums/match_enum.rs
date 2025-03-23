#[derive(Debug)]
enum OperatingSystem {
    Window,
    MacOS,
    Linux,
}

pub fn main() {
    println!("Match with enums:\n");
    let my_computer = OperatingSystem::MacOS;
    let years_ago = years_since_release(&my_computer);

    println!("I can do a match with the type variant of a Enum and 'play' with these values");
    println!(
        "For example:

    enum OperatingSystem {{
        Window,
        MacOS,
        Linux
    }}b 

    let os: &OperatingSystem = &OperatingSystem::Linux;
    match os {{
        OperatingSystem::Window => 39,
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }}
    "
    );

    println!("{:?} was created {} years ago", my_computer, years_ago);
    println!("---\n");
}

fn years_since_release(os: &OperatingSystem) -> i32 {
    match os {
        OperatingSystem::Window => 39,
        OperatingSystem::MacOS => 23,
        OperatingSystem::Linux => 34,
    }
}
