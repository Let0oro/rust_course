/**
=== SETTER & GETTER ===

An associated constant is a constant declared within the trait.
A constant is a name for a fixed, immutable value.

 - The same concept and flexibility than fn attached to traits

At start, we can't use property that we are not sure about will be implemented in every case:
For example:
```
trait Taxable {
    ...
    self.amount -> exists if we declare a enum with the trait Taxable or a struct without amount property? No
    ...
```
 - Solution -> Getter method, retrieves a piece of data. It 'gets' a piece of state. Traits supports it
The traits cannot guarantee fields, but can guarantee methods -> "fn amount..." === "self.amount" NO / "self.amount()" YES

But we get an abstract value forma  function, and we cant overwrite the real property. What if we want to duplicate the amount property? -> Setter
 - Setter is a method that writes a piece of data. It "sets" a piece of state


=== SUPERTRAITS ===

 - Trait from which another trait inherits functionality.
 - The parent is called the supertrait and the child is called de subtrait.
 - Any type that implements a subtrait must implement its supertrait.

*/

mod display;

/// Supertrait
trait Investment<T>
where T:
{
    /// Getter
    fn amount(&self) -> T;

    ///Setter
    /// ```fn set_amount(&mut self, new_amount: T);``` \

    /** Setter trait implementation (always will run equal)
    ```
    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
    ```
    */
    fn double_amount(&mut self);
}

/// Associated constants in a trait \
/// "trait name_of_trait: name_of_supertrait" -> Inheritance \
/// We can overwrite the methods of the supertrait in each subtrait to more flexibility
trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;



    /// Getter trait implementation (always will run equal)
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }

}

#[derive(Debug)]
struct Income {
    amount: f64,
}

/// We need to implement the traits separated,
/// but if the supertrait is not implemented in the struct affected by the subtrait, we will get an error.
/// Also, we can only implement the supertrait, not the subtrait, and not generate errors
/// "A child needs his parent, but the parent not need his child"
impl Investment<f64> for Income {
    fn amount(&self) -> f64 { self.amount }
    // fn set_amount(&mut self, new_amount: f64) {
    //     self.amount = new_amount;
    // }
    fn double_amount(&mut self) {
        self.amount = self.amount * 2.0;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 { self.value }
    // fn set_amount(&mut self, new_amount: f64) {
    //     self.value = new_amount;
    // }
    fn double_amount(&mut self) {
        self.value = self.value * 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
}

struct QualityTime {
    minutes: u32,
}

/// Only supertrait implementing
impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 { self.minutes }
    // fn set_amount(&mut self, new_amount: f64) {
    //     self.minutes = new_amount;
    // }
    fn double_amount(&mut self) {
        self.minutes = self.minutes * 2;
    }
}

pub fn main() {
    let income = Income {amount: 50000.50};
    println!("Total tax owned: {:.2}€", income.tax_bill());

    let bonus = Bonus {value: 10000.23};
    println!("Bonus tax owner: {:.2}", bonus.tax_bill());


}