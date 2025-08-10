#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let coin1 = Coin::Dime;
    println!("The value of a {:?} is: {} cents", coin1, value_in_cents(&coin1));
    let coin2 = Coin::Quarter;
    println!("The value of a {:?} is: {} cents", coin2, value_in_cents(&coin2));
    let coin3 = Coin::Penny;
    println!("The value of a {:?} is: {} cents", coin3, value_in_cents(&coin3));
    let coin4 = Coin::Nickel;
    println!("The value of a {:?} is: {} cents", coin4, value_in_cents(&coin4));
}