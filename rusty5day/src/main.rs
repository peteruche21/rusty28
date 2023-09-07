use std::ops::Add;

// generics
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// traits
trait Overview {
    fn overview(&self) -> String {
        String::from("this is me learning rust")
    }
}

struct Course {
    title: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self) {
        println!("dropping: {}", self.author);
    }
}

struct AnotherCourse {
    title: String,
    author: String,
}

struct LastCourse {
    title: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{} {}", self.author, self.title)
    }
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{} {}", self.author, self.title)
    }
}

impl Overview for LastCourse {}

impl<T, U> Add for Point<T, U>
where
    T: Add<Output = T>,
    U: Add<Output = U>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let point_int = Point::<i32, i32> { x: 5, y: 8 };
    let point_str = Point::<i32, &str> { x: 10, y: "john" };

    let course1 = Course {
        title: String::from("rust"),
        author: String::from("peter anyaogu"),
    };
    let another_course = AnotherCourse {
        title: String::from("cairo"),
        author: String::from("starknet"),
    };

    let last_course = LastCourse {
        title: String::from("nullable"),
        author: String::from("any"),
    };

    println!("{}", course1.overview());
    println!("{}", another_course.overview());
    println!("{}", last_course.overview());

    call_overview(&course1);
    call_overview_the_second_way(&course1);

    drop(course1);

    let coord = Point { x: 5.0, y: 5.0 };
    let coord2 = Point { x: 1.0, y: 2.0 };

    let sum = coord + coord2;
    println!("sum of coordinates are: {:?}", sum);
}

fn call_overview(item: &impl Overview) {
    println!("overview {}", item.overview());
}

fn call_overview_the_second_way<T: Overview>(item: &T) {
    println!("overview the second way {}", item.overview());
}
