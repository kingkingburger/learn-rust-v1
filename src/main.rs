// use std::{cmp::Ordering, io};

// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     // println!("The secret number is {secret_number}");

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win1");
//                 break;
//             }
//         }
//     }
// }
mod while_test;

use while_test::while_test;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for num in a {
        println!("This is array {num}");
    }
    // while_test();
    // another_function();
    // print_labeled_measurement(5, 'h');
    // express();

    let s1 = String::from("hello");

    // takes_ownership(s);
    let len = calculate_length(&s1); // 함수에 튜블 형태로 소유권 다시 넘겨주기
    println!("{}", len);
    // println!(s); s의 메모리는 takes_ownership에서 해제됨

    let x = 5;

    makes_copy(x);

    let mut tt = String::from("hello");
    change(&mut tt);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn calculate_length(s: &String) -> usize{
    s.len()
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn another_function() {
    println!("another function");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn express() -> i32 {
    let y = {
        let x = 3;
        return x + 1;
    };
}
