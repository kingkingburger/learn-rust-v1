use std::{
    fs::File,
    io::{self, Read},
};

use rand::Error;

use crate::v1::learn_hash::learn_hash;

mod v1;

fn main() {
    let greeting_file_result = File::open("hello.txt").expect("should include this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?
    Ok(username)
}
