// Create a struct called Car with the fields: mpg, color, and top_speed. Once the struct is
// created, implement the following methods: set_mpg, set_color, and set_top_speed. Once you
// have created these methods, create a car, provide it default values, and then set the mpg,
// color, and top speed and then print them out.
#[derive(Debug)]
struct Car {
    mpg: u32,
    color: String,
    top_speed: u64,
}

impl Car {
    fn set_mpg(&mut self, mpg: u32) {
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: &str) {
        self.color = color.to_string();
    }
    fn set_top_speed(&mut self, top_speed: u64) {
        self.top_speed = top_speed;
    }
}

fn main() {
    let mut mercedes = Car {
        mpg: 24,
        color: String::from("red"),
        top_speed: 2_400,
    };
    println!("before upgrade: {:?}", mercedes);

    mercedes.set_color("white");
    mercedes.set_mpg(30);
    mercedes.set_top_speed(2_100);

    println!("after upgrade: {:?}", mercedes);
}
