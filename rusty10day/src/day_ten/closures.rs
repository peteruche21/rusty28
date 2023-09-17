#[derive(Debug)]
struct City {
    city: String,
    population: u64,
}

fn pop_helper(pop: &City) -> u64 {
    pop.population
}

pub fn closeure() {
    let a = City {
        city: String::from("A"),
        population: 100,
    };
    let b = City {
        city: String::from("B"),
        population: 80,
    };
    let c = City {
        city: String::from("C"),
        population: 70,
    };
    let d = City {
        city: String::from("D"),
        population: 60,
    };
    let e = City {
        city: String::from("E"),
        population: 50,
    };

    let mut vec: Vec<City> = Vec::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);
    vec.push(d);
    vec.push(e);

    vec.sort_by_key(pop_helper);
    vec.sort_by_key(|p| p.population);
    println!("{:?}", vec);

    let add = |x: i32| -> i32 { x + 1 };
    // the same thing as:
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let add_v2 = |x| x + 1;
    add_v2(5 as i32); // correct
                      // add_v2(7 as i128); // incorrect
}
