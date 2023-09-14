use std::fs::{rename, File};
use std::io::{Error, ErrorKind};

fn main() {
    // panic!("Hello, world!");

    // let vec = vec![1];
    // vec[10];

    let file = File::open("error.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("error.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("unable to create your file!"),
            },
            _ => panic!("something occured, try again!"),
        },
    };

    let file2 = File::open("error.txt").unwrap();
    let file3 = File::open("error.txt").expect("Error opening the file");

    // let test = open_file();
}

// fn open_file() -> Result<File, Error> {
//     let file = File::open("error.txt")?;
//     Ok(file)
// }

enum Result<T, E> {
    Ok(T),
    Err(E),
}
