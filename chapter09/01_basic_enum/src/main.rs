// This is a simple example of defining and using an enum in Rust.
// The enum `IpAddrKind` represents different kinds of IP addresses.
// It has two variants: `V4` for IPv4 addresses and `V6` for IPv6 addresses.

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IPv4: {:?}, IPv6: {:?}", four, six);
}
