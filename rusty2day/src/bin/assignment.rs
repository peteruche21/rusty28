fn main() {
    let mut initial_vector = vec![1, 3, 5, 7];
    check_and_print_vector(&mut initial_vector);

    let mut un_mutable_int: i8 = 5;
    println!("before {}", un_mutable_int);
    add_two(un_mutable_int);
    un_mutable_int = 3;
    println!("after {}", un_mutable_int);
}

// Create a function that takes one argument called val that is of type Vec with the values: 1,3,5,7.
// Inside of this function check the first value of the vector and see if it is equal to one. If the
// value is equal to one, then return true, otherwise return false. Next add the value 15 to the vector.
// Print out the vector to confirm your results.

fn check_and_print_vector(val: &mut Vec<i32>) -> bool {
    let is_one: bool;

    if val[0] == 1 {
        is_one = true;
    } else {
        is_one = false;
    }

    val.push(15);
    println!("{:?}", val);

    is_one
}

// Create a function called "add_two". This function is going to take one parameter that is i8 and
// add two to it. For the function, do you have to pass the value by reference in order for you to
// maintain ownership of it inside of main?

fn add_two(x: i8) {
    x + 2;
}
