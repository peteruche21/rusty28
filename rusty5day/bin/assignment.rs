fn main() {
    let mut toyota: Motorcycle<u32, u64> = Motorcycle {
        mpg: 24,
        color: String::from("black"),
        top_speed: 2_400,
    };
    println!("before upgrade: {:?}", toyota);

    toyota.set_color("red");
    toyota.set_mpg(30);
    toyota.set_top_speed(2_100);

    println!("after upgrade: {:?}", toyota);

    print_generic(&toyota);
}

// Modify the solution to the Section 3 assignment by creating a Trait that has the set_mpg,
// set_color, and set_top_speed methods. Then create a Motorcycle struct with the same fields
// as the Car struct: mpg, color, and top_speed. Now implement your Trait on both the Car and
// Motorcycle struct. Print out the results to confirm a working solution.

trait Vehicle<T, U> {
    fn get_mpg(&self) -> &T;
    fn set_mpg(&mut self, mpg: T);
    fn set_color(&mut self, color: &str);
    fn set_top_speed(&mut self, top_speed: U);
}

#[derive(Debug)]
struct Motorcycle<T, U> {
    mpg: T,
    color: String,
    top_speed: U,
}

impl<T, U> Vehicle<T, U> for Motorcycle<T, U> {
    fn get_mpg(&self) -> &T {
        &self.mpg
    }
    fn set_mpg(&mut self, mpg: T) {
        self.mpg = mpg;
    }
    fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }
    fn set_top_speed(&mut self, top_speed: U) {
        self.top_speed = top_speed;
    }
}

// Create a simple print function that uses Generic T. This Generic T will need to implement
// std::fmt::Debug depending on the values you pass in. Our function takes one parameter of type
// T. Our function will then print out the value that is passed in.

fn print_generic<T: std::fmt::Debug, U>(vehicle: &impl Vehicle<T, U>) {
    println!("{:?}", vehicle.get_mpg());
}
