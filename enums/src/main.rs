enum IppAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    // Each variant can have different types and amounts of associated data
    _Quit,                       // This is an variant with no data
    _Move { x: i32, y: i32 },    // This is an anonymous struct
    Write(String),               // This is a String
    _ChangeColor(i32, i32, i32), // This is a tuple
}
// We can also define methods for enums like we do for structs
impl Message {
    fn call(&self) {
        // method body would be defined here
        print!("This is a method for the Message enum");
    }
}

struct IppAddr {
    kind: IppAddrKind,
    address: String,
}

fn main() {
    // We can create instances of the enum variants
    let m: Message = Message::Write(String::from("hello"));
    m.call();

    // We can initialize the struct with the enum variants
    let localhost_v4: IppAddr = IppAddr {
        kind: IppAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };

    let localhost_v6: IppAddr = IppAddr {
        kind: IppAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };

    route(localhost_v4);
    route(localhost_v6);
}

fn route(ip_addrs: IppAddr) {
    // We match the value of the ip_kind variable
    match ip_addrs.kind {
        IppAddrKind::V4(a, b, c, d) => {
            println!("The IP address is: {}.{}.{}.{}", a, b, c, d);
        }
        IppAddrKind::V6(addr) => {
            println!("The IP address is: {}", addr);
        }
    }
    println!("The IP address is: {}", ip_addrs.address);
}
