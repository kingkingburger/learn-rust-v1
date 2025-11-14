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

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for num in a {
        println!("This is array {num}");
    }
    while_test();
    // another_function();
    // print_labeled_measurement(5, 'h');
    // express();
}

fn while_test() {
    let a = [10,20,30,40,50];
    let mut index = 0;

    while (index < 5){
        println!("the value is: {}", a[index]);
        index += 1; 
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
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
