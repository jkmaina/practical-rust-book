// In main.rs
mod front_of_house;
use front_of_house::hosting;
fn main() {
    hosting::add_to_waitlist();
}
