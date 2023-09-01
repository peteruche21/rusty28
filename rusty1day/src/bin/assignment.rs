fn main() {
    get_modulo();
    operate_vector();
    let mut var: String = String::from("hello");
    var = concat_string(var);
    println!("concatenated string: {}", var);
    control_flow(45);
    control_flow(55);
    control_flow(15);
    control_flow(1);
}

// 1. Create three variables with the names: val1, val2, and ans. We want to perform a simple
// operation of generating the modulo of val1 and val2. Set val1 to 5 and val2 to 2. Assign the
// answer to the ans variable. Before executing your code, what do you think the answer will be?

fn get_modulo() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("modulo: 5 % 2: {}", ans);
}

// Create a vector and put in the values "2, 4, 6, 8, 10". Once you have created the vector perform
// the following: print out the current values, remove the value 10, add the value 12, and then print
// the vector back out to confirm your results.

fn operate_vector() {
    let mut vec: Vec<i32> = vec![2, 4, 5, 6, 10];
    println!("Vector are: {:?}", vec);

    vec.pop();
    vec.push(12);

    println!("New Vector is: {:?}", vec);
}

// Create a function called "concat_string". Create a string variable and assign the value "Hello"
// to it. The function is going to take one argument that is of type string and is going to return a
// String. Inside this function, concatenate the string " World". Print out the results in main() to
// confirm your results.

fn concat_string(var: String) -> String {
    var + " world"
}

// Create a function called control_flow. This is going to take one argument that is an integer.
// Based on this integer, print out the following: "The value is one", "The value is greater than 50",
// "The value is less than 25", or "The value is greater than 25 but less than 50".

fn control_flow(num: u32) {
    if num == 1 {
        println!("The value is one");
    }
    if num > 50 {
        println!("The value is greater than 50");
    } else if num < 25 {
        println!("The value is less than 25")
    } else if num > 25 && num < 50 {
        println!("The value is greater than 25 but less than 50");
    }
}
