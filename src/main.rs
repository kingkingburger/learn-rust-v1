mod while_test;

use while_test::while_test;

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

fn main() {
    let user1 = User {
        active: true,
        username: String::from("sdfasdf"),
        email: String::from("asdf@gmail.com"),
        sign_in_count: 1,
    };

    // build_user(email, username);

    let user2 = User {
        email: String::from("anthor"),
        ..user1
    };

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

    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    let hello = &s[0..5];
    let world = &s[6..11];
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
