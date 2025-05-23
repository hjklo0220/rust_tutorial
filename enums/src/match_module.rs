#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn match_module_main() {
    println!("in match_module");

    let coin = Coin::Penny;
    let value = value_in_cents(&coin);
    println!("{}", value);

    let coin2 = Coin::Quarter(UsState::Alaska);
    let value2 = value_in_cents(&coin2);
    println!("{}", value2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximun is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximun is configured to be {max}")    
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
    println!("count = {count}");

    println!("============");
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}