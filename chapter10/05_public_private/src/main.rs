// This code demonstrates the use of public and private fields in a module in Rust.
// The `Breakfast` struct has a public field `toast` and a private field `seasonal_fruit`.
// The `summer` function is used to create an instance of `Breakfast` with a specific toast type.
// The main function shows how to modify the public field but not the private one.

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
fn main() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    // We can modify the toast field because it's public
    meal.toast = String::from("Wheat");
    
    // This would cause an error because seasonal_fruit is private:
    // meal.seasonal_fruit = String::from("blueberries");
}
