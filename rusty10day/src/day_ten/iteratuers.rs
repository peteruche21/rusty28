#[derive(Debug)]
struct Item {
    name: String,
}

#[derive(Debug)]
struct RangeCheck {
    start: u32,
    end: u32,
}

impl Iterator for RangeCheck {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item> {
    items.into_iter().filter(|i| i.name == product).collect()
}
pub fn iter_tuer() {
    let vec = vec![1, 2, 3];

    for val in vec.iter() {
        println!("{}", val);
    }

    let vec2 = vec![1, 1, 2, 3];

    let mut iter = (&vec2).into_iter();

    while let Some(v) = iter.next() {
        println!("{}", v)
    }

    let mut vec: Vec<Item> = Vec::new();
    vec.push(Item {
        name: String::from("water"),
    });
    vec.push(Item {
        name: String::from("shoes"),
    });
    vec.push(Item {
        name: String::from("clothes"),
    });
    vec.push(Item {
        name: String::from("mat"),
    });

    let itm = check_inventory(vec, String::from("water"));
    println!("{:?}", itm);

    let mut range = RangeCheck { start: 0, end: 4 };
    for r in range {
        println!("{}", r)
    }
}
