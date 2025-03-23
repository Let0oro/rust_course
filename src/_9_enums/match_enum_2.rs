enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature");
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}");
            }
            LaundryCycle::Delicate(arbitrary_fabric_type) => {
                println!("Running the laundry with a delicate cycle for {arbitrary_fabric_type}");
            }
        }
    }
}

pub fn main() {
    println!("Match enums II\n");

    println!(
        "We can pass to a match not tonly the enum variants, also the values associated with and use into the match arm function"
    );

    println!(
        "
    enum LaundryCycle {{
        Cold,
        Hot {{ temperature: u32 }},
        Delicate(String),
    }}

    match cycle <&LaundryCycle> {{
        LaundryCycle::Cold => {{
            println!(\"Running the laundry with cold temperature\");
        }},
        LaundryCycle::Hot {{ temperature }} => {{
            println!(\"Running the laundry with a temperature of {{temperature}}\");
        }},
        LaundryCycle::Delicate(arbitrary_fabric_type) => {{
            println!(\"Running the laundry with a delicate cycle for {{arbitrary_fabric_type}}\");
        }}
    }}
    "
    );

    // wash_laundry(&LaundryCycle::Cold);
    // wash_laundry(&LaundryCycle::Hot { temperature: 100 });
    // wash_laundry(&LaundryCycle::Delicate(String::from("Silk")));

    println!(
        "Other way is to define a method for the Enum, like this:
    impl LaundryCycle {{
        fn wash_laundry(&self) {{
            match self {{...same functionality wash_laundry fn match}};
        }}
    }}

    LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot {{temperature: 80}}.wash_laundry();
    LaundryCycle::Delicate(String::from(\"Silk\")).wash_laundry();
    "
    );

    LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot { temperature: 80 }.wash_laundry();
    LaundryCycle::Delicate(String::from("Silk")).wash_laundry();

    println!("---\n");
}

// fn wash_laundry(cycle: &LaundryCycle) {
//     match cycle {
//         LaundryCycle::Cold => {
//             println!("Running the laundry with cold temperature");
//         },
//         LaundryCycle::Hot { temperature } => {
//             println!("Running the laundry with a temperature of {temperature}");
//         },
//         LaundryCycle::Delicate(arbitrary_fabric_type) => {
//             println!("Running the laundry with a delicate cycle for {arbitrary_fabric_type}");
//         }
//     }
// }
