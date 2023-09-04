// name field structs
struct User {
    name: String,
    active: bool,
    count: u32,
}

// tuple structs
#[derive(Debug)]
struct Cords(i32, i32);

// unit struct
struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

struct MyString<'a> {
    text: &'a str,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    let user1 = User {
        active: true,
        name: String::from("peter"),
        count: 0,
    };
    println!("{}", user1.name);
    let user2 = build_user("uchenna".to_string());
    println!("{}", user2.name);

    let ll = Cords(102, 43);
    println!("{:?}", ll);

    let mut sq = Square {
        width: 10,
        height: 10,
    };

    println!("{}", sq.area());
    println!("{}", sq.get_width());
    sq.change_width(20);
    println!("{}", sq.get_width());

    // dangling reference
    let r;

    {
        let x = 5;
        r = &x;
    }

    let str1 = String::from("this is my lifetime");
    let x = MyString {
        text: str1.as_str(),
    };
    println!("{}", x.text);

    // static lifetimes
    let j: &'static str = "i am a static lifetime";
}

fn build_user(name: String) -> User {
    User {
        name,
        active: true,
        count: 0,
    }
}

// lifetime

fn lifetime_eg<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
