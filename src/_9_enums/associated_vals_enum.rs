#[derive(Debug)]
enum PaymentMethodType {
    CreditCard(String),
    DebitCard(Credentials),
    PayPal(String, String),
    Cash,
}

#[derive(Debug)]
struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub fn main() {
    println!("Associated values in enums\n");

    let visa = PaymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));

    println!("How to associate values:
   - With a tuple -> let visa = (PaymentMethodType::CreditCard, String::from(\"0034-5678-9012-3456\")); - This way is too common, and Rust allow to associate values with some like this but in the own enum:
   
   - In the own enum (with tuples) 'tupla variants':
    enum PaymentMethodType {{
      CreditCard(String),
      DebitCard(Credentials),
      PayPal(String, String),
      Cash, // For separatedly type purposes
    }}
    is not obligatory to store data in every variants, nor every have the same data or data type.
    The representation will be -> 'let visa = PaymentMethodType::CreditCard(String::from(\"0034-5678-9012-3456\"));'

   - 'Struct variant': 
      -> stores associated data in fields rather than by position. Each piece of data has an associated name.

      Example: 
      let mastercard_credentials = Credentials::new(
          String::from(\"bob@mail.com\"),
          String::from(\"pass123\"),
      )
      let mastercard = PaymentMethodType::DebitCard(mastercard_credentials);
      
       * This way is also too common, and Rust allow to associate values with some like this but in the own enum:
      ...
      DebitCard {{ username: String, password: String }}
      ...
      So the new way to declare it will be:
      let paypal = PaymentMethodType::DebitCard {{ 
        username: \"bob@mail.com\", 
        passoword: \"12345\" 
      }};
  ");

    println!("visa: {:#?}", visa);

    let mastercard_credentials =
        Credentials::new(String::from("bob@mail.com"), String::from("pass123"));
    let mastercard = PaymentMethodType::DebitCard(mastercard_credentials);
    println!("mastercard: {:#?}", mastercard);

    let my_paypal_method =
        PaymentMethodType::PayPal(String::from("bob@email.com"), String::from("password"));

    println!("my_paypal_method: {:#?}", my_paypal_method);

    println!("---\n");
}
