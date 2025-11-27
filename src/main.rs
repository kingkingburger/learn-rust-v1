mod while_test;

use while_test::while_test;

fn main() {
    let rect1 = Rectangl {
        width: 30,
        height: 50,
    };

    println!("rect is {:?}", rect1);
    println!("rect is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangl {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangl {
    width: u32,
    height: u32,
}

fn area3(rectangl: &Rectangl) -> u32 {
    rectangl.width * rectangl.height
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");

    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2);
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

    let r3 = &mut s; // 문제없음
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
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
