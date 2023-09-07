enum Pet {
    Dog,
    Cat,
    Fish,
}

impl Pet {
    fn what_am_i(self) -> &'static str {
        match self {
            Pet::Dog => "i am a dog",
            Pet::Cat => "i am a cat",
            Pet::Fish => "I am a fish",
        }
    }
}

enum IPAddrKind {
    V4(String),
    V6,
}

struct IpAddr {
    kind: IPAddrKind,
    address: String,
}

fn main() {
    let dog = Pet::Dog;
    println!("{}", dog.what_am_i());

    let home = IpAddr {
        kind: IPAddrKind::V4(String::from("192.168.0.1")),
        address: String::from("192.168.0.1"),
    };
    let loopBack = IpAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };

    let some_num = Some(5);
    let some_str = Some("a string");
    let nothing: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let something = Some(Pet::Cat);

    if let something = Some(Pet::Cat) {
        println!("{}", "found a cat");
    }

    let x = Some(5);
    let y = 10;

    match y {
        2 | 5 => println!("{}", y),
        _ => println!("default"),
    };

    match y {
        1..=10 => println!("{}", y),
        _ => println!("default"),
    };

    match x {
        Some(5) => println!("matches!"),
        Some(x) if x == y => println!("matches also!"),
        _ => println!("default"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => Some(0),
    }
}
