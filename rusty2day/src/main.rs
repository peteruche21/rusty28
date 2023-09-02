fn main() {
    // stack
    let num: i32 = 25; // does not change at runtime time
                       // heap
    let mut str: String = "dynamic".to_string(); // can change at runtime
                                                 // how?
    str.push_str(", world"); // changes

    let x = vec!["tyler".to_string()];
    let y = x;
    println!("{:?}", y);

    let a = vec!["peter".to_string()];
    let b = a.clone();
    let c = b.clone();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    let g = 1;
    let h = g;

    let mut mutable = String::from("main");
    let non_mutable = String::from("minor");

    mutable.push_str("mut");
    // non_mutable.push_str("non mut");

    change_string(&mut mutable);
}

fn change_string(a_str: &mut String) {
    a_str.push_str(" subject");
}
