#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}

pub fn main() {
    println!("Build Pattern\n");

    println!(
        " - A design patter is a recommended way to write or structure code to solve specific problems"
    );

    let mut computer = Computer::new(String::from("M3 Max"), 64, 2);

    println!("My Computer Struct -> {:#?}", computer);
    println!(
        "Shoud be better to add Computer upgrades (or set) to all namespaces or values defining on his struct"
    );
    println!(
        " * If we return 'self' at every upgrade fn, we can upgrade concatenally the values of our Computer instances"
    );

    println!(
        " * From this: 
  computer.upgrade_cpu(String::from(\"M4 Max\"));
  computer.upgrade_memory(128);

  to this:
  computer.upgrade_hard_drive_capacity(4).upgrade_memory(3);
  "
    );

    computer.upgrade_cpu(String::from("M4 Max"));
    computer.upgrade_memory(128);
    println!("Stats: {computer:#?}");

    computer.upgrade_hard_drive_capacity(4).upgrade_memory(3);
    println!("Stats: {computer:#?}");

    println!("---\n");
}
