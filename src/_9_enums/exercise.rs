/*
In the `main` function, create 3 instances of Subscription,
each one with a different variant. Invoke the `summarize`
method on each instance.
*/

#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32), // (the price per month, the number of months)
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site")
            }
            Subscription::Basic(price, months) => {
                println!(
                    "You have limited access to the site's premium features for {price}$ for {months} months"
                )
            }
            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is {tier:?}"
                )
            }
        }
    }
}

pub fn main() {
    println!("Exercise:\n");
    Subscription::Free.summarize();
    Subscription::Basic(18.5, 10).summarize();
    Subscription::Premium { tier: Tier::Gold }.summarize();
    println!("---\n");
}
