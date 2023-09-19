use std::rc::Rc;

pub fn task() {
    //Question 1: Create a variable on the stack and a variable on the heap. Multiply their values
    // and print out the results.

    let stack_var: i32 = 25;

    let heap_var = Box::new(stack_var);

    println!("stack * heap = {}", stack_var * *heap_var);

    //Question 2: Create a variable that holds a String

    let str_var = String::from("variable");

    {
        //Create a reference counting smart pointer that points to the above String.

        let counter = Rc::new(str_var);

        let rc_value = Rc::strong_count(&counter);

        //Print out how many references the smart pointer has.
        println!("{}", rc_value);

        //Code block
        {
            //Create another reference counting smart pointer that points to our first smart pointer
            let counter_two = counter.clone();

            let rc_value = Rc::strong_count(&counter_two);

            //Print out how many references each smart pointer has
            println!("{}", rc_value);
        }
        //What value is dropped here?
        // counter_two is dropped
        //Print out how many references out first smart pointer has
        println!("{}", rc_value);
    } //What value is dropped here?
      // couter is dropped
      //Comment out the line below. What do you think will happen when you try to run the program now?
      // println!("rc_value: {}", rc_value);

    // will panic! as the counter is out of scope
}
