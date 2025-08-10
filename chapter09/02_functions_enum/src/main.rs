// This program defines an enum for IP address kinds and a function to route based on the kind
// It demonstrates how to use enums in Rust for pattern matching.
// The program will print messages based on the IP address kind passed to the route function.

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing IPv4..."),
        IpAddrKind::V6 => println!("Routing IPv6..."),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}
