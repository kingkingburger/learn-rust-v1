use std::fs::File;

use crate::v1::learn_hash::learn_hash;

mod v1;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
