use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let mut nums = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);
    nums.push(4);

    let pop = nums.pop();
    println!("{:?}", pop);

    let two = nums[1]; // copy
    println!("{}", two);

    let one = nums.first();
    println!("{:?}", one);

    println!("{}", nums.len());
    println!("{}", nums.is_empty());

    nums.insert(0, 10);
    nums.insert(3, 12);
    nums.insert(2, 25);

    nums.remove(3);

    nums.sort();
    println!("{:?}", nums);

    nums.reverse();
    println!("{:?}", nums);

    nums.shuffle(&mut thread_rng());
    println!("{:?}", nums);

    let mut bheap = BinaryHeap::new();

    bheap.push(1);
    bheap.push(18);
    bheap.push(20);
    bheap.push(5);

    bheap.pop();

    println!("{:?}", bheap);
    // peak will leave the value, pop will remove the value and return it to us
    println!("{:?}", bheap.pop());
    println!("{:?}", bheap.peek());
    println!("{:?}", bheap);

    let mut hm = HashMap::new();

    hm.insert(1, 1);
    hm.insert(5, 2);
    hm.insert(30, 3);
    let old = hm.insert(30, 5);

    println!("{:?}", old);
    println!("{:?}", hm);
    println!("{:?}", hm.contains_key(&5));
    println!("{:?}", hm.contains_key(&21));
    println!("{:?}", hm.get(&30));

    let ret = hm.remove(&30);
    println!("{:?}", hm);

    let ret_entry = hm.remove_entry(&1);
    println!("{:?}", hm);

    hm.clear();
    println!("{:?}", hm.is_empty());

    let mut sert = HashSet::new();

    sert.insert("john");
    sert.insert("peter");
    sert.insert("simon");
    sert.insert("mark");

    for entry in sert.iter() {
        println!("{:?}", entry);
    }

    let mut sert2 = HashSet::new();
    sert2.insert("peter");
    sert2.insert("emma");
    sert2.insert("unity");
    sert2.insert("lucy");

    for x in sert.intersection(&sert2) {
        println!("{:?}", x);
    }

    let intersection = &sert & &sert2;
    println!("{:?}", intersection);

    let unity = &sert | &sert2;
    println!("{:?}", unity);

    let diff = &sert - &sert2;
    println!("{:?}", diff);
}

// cargo modules generate tree --with-types
