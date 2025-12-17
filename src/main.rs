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
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
