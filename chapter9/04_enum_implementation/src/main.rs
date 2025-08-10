enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(127, 0, 0, 1) => true,
            IpAddr::V6(s) if s == "::1" => true,
            _ => false,
        }
    }
}
fn main() {
    let loopback_v4 = IpAddr::V4(127, 0, 0, 1);
    let loopback_v6 = IpAddr::V6(String::from("::1"));
    let non_loopback_v4 = IpAddr::V4(192, 168, 1, 1);
    let non_loopback_v6 = IpAddr::V6(String::from("2001:db8::ff00:42:8329"));

    println!("Is loopback_v4 a loopback address? {}", loopback_v4.is_loopback()); // true
    println!("Is loopback_v6 a loopback address? {}", loopback_v6.is_loopback()); // true
    println!("Is non_loopback_v4 a loopback address? {}", non_loopback_v4.is_loopback()); // false
    println!("Is non_loopback_v6 a loopback address? {}", non_loopback_v6.is_loopback()); // false
}
