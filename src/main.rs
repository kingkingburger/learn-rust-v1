use std::{fs::File, io::ErrorKind};

use crate::v1::learn_hash::learn_hash;

mod v1;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("not creating the file {:?}", e),
            },
            other_error => {
                panic!("opening the file: {:?}", other_error)
            }
        },
    };
}
