#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered => {
                println!("Your requested online ordered");
            }
            OnlineOrderStatus::Shipped | OnlineOrderStatus::Delivered => {
                println!("Your item has been sent to you");
            }
            other_status => {
                println!("Your item is {{other_status:?}}.");
            }
        }
    }
}

pub fn main() {
    println!("Match enums III\n");

    println!(
        "We can match not only an arm, also two or more arm possibilities are possible:
       match example <arm1, arm2, arm3> {{
            arm1 => println!(\"Arm1\");
            arm2 | arm3 => println!(\"Arm2 and 3 are selected\");
            arbitrary_name => println!(\"Your selection is {{arbitrary_name}}\");
       }}
    "
    );

    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();

    println!("---\n");
}
