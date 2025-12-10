use std::io::{self, Write};

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // 호밀(Rye) 토스트를 곁들인 여름철 조식 주문
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 다음 라인의 주석을 해제하면 컴파일되지 않습니다
    // 계절 과인은 조회나 수정이 허용되지 않습니다.
    // meal.seasonal_fruit = String::from("blueberries");
}

// not working, customer 스코프에서만 사용할 수 있음 eat_at_restaurant 까지 못감.
mod customer {
    // pub fn eat_at_restaurant() {
    //     hosting::add_to_waitlist();
    // }
}
