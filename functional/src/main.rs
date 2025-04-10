use core::num;
use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveway1 = store.giveway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveway1
    );

    let user_pref2 = None;
    let giveway2 = store.giveway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    let result1 = add_one_v3(5);
    let result2 = add_one_v4(5);
    println!("{}, {}", result1, result2);

    let number = vec![1, 2, 3, 4, 5];
    let incremented1: Vec<i32> = number.iter().map(|x| x + 1).collect();
    println!("{:?}", incremented1);

    let example_closure = |x| x;
    let s = example_closure(String::from("helolo"));
    // let n = example_closure(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure {:?}", list);

    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);

    let mut borrows_mutably = || {
        println!("push 7: {:?}", list2.push(7));
    };

    borrows_mutably();
    println!("After calling closure {:?}", list2);

    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);

    thread::spawn(move || println!("From thread: {:?}", list3))
        .join()
        .unwrap();
    // println!("After calling closure: {:?}", list3);

    let mut list4 = [
        Rectangle { w: 10, h: 1 },
        Rectangle { w: 3, h: 5 },
        Rectangle { w: 7, h: 12 },
    ];

    list4.sort_by_key(|r| r.w);
    println!("{:?}", list4);

    let mut list5 = [
        Rectangle { w: 10, h: 1 },
        Rectangle { w: 3, h: 5 },
        Rectangle { w: 7, h: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list5.sort_by_key(|r| {
        sort_operations.push(value.clone());
        println!("v{}. r.w{}. r.h{}", value, r.w, r.h);
        r.w
    });
    println!("{:?},\n {:?}", list5, sort_operations);

    let mut num_sort_operations = 0;
    list5.sort_by_key(|r| {
        num_sort_operations += 1;
        r.w
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);



}

#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}
