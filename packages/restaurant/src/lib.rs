// mod customer_experience {
//     mod front_of_house {
//         pub mod hosting {
//             pub fn add_to_waitlist() {}
//         }
//     }
//     pub fn eat_at_restaurant() {
//         // 절대 경로
//         // crate::customer_experience::front_of_house::hosting::add_to_waitlist();
    
//         // 상대 경로
//         front_of_house::hosting::add_to_waitlist();
//     }
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

mod dining {
    use crate::front_of_house;

    pub fn eat_at_restaurant() {
        // 절대 경로
        crate::front_of_house::hosting::add_to_waitlist();
    
        // 상대 경로
        front_of_house::hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_oder();
        super::deliver_order();
    }

    fn cook_oder() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();
        super::hosting::add_to_waitlist();
    }
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

mod front_of_house_2 {
    pub mod hosting_2 {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house_2::hosting_2;

pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
}