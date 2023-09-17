// Create a vector with the values 1, 3, 5, 7, and 9. Then use an iterator and a closure to multiply all
// of the values by 10 and store the result in another vector. Print out the vector to confirm your
// results.

pub fn assignment() {
    let vec = vec![1, 3, 5, 7, 9];

    let mut new_vec = Vec::<i32>::with_capacity(5);

    let multiply = |x: i32| -> i32 { x * 10 };

    for num in vec.iter() {
        new_vec.push(multiply(*num));
    }

    println!("old vec: {:?}", vec);
    println!("new vec: {:?}", new_vec);
}
