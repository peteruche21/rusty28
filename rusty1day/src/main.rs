use std::vec;

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    nums.push(2);

    // print!("{:?}", nums);

    let mut text = Vec::new();
    text.push("value");
    text.push("value 2");

    // println!("{:#?}", text);

    text.reverse();
    println!("{:#?}", text);

    let vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());

    let vetc2: Vec<i32> = (0..5).collect();
    println!("{:?}", vetc2);

    // slices from the vetc pointer

    // ordinary ref points to a single value
    // slice ref points to a range of consecutive values.
    // both of which are non owning
    let sv: &[i32] = &vetc2;
    println!("{:?}", sv);

    // lets slice a specific range
    let sv2: &[i32] = &vetc2[0..4];
    println!("{:?}", sv2);

    let name: String = String::from("peter");
    let course: String = "rust".to_string();
    let new_name: String = name.replace("peter", "john");

    println!("{} {} {}", name, course, new_name);

    // str slices
    let ssss = "slice";
    let strrr = ssss.to_string();
    let sss = &strrr;

    println!("{}", sss);

    snake_cased_functions("this is an argument");

    // conditions
    if 1 > 0 {
        println!("{}", true)
    } else {
        println!("{}", false)
    };

    loops_in_rust();
}

// functions

fn snake_cased_functions(phrase: &str) {
    println!("{}", phrase);
    let b = gcd(10, 50);
    println!("{}", b);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn loops_in_rust() {
    let mut num = 0;
    'counter: loop {
        let mut decrease = 5;
        loop {
            print!("{} ", decrease);
            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }

    // for loops

    let vec: Vec<i8> = (0..3).collect();

    for el in vec {
        print!("{} ", el);
    }

    for num in (1..4).rev() {
        println!("{} ", num);
    }
}
