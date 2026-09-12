use std::ops::Div;

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

fn plus_one(n: Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(n: i32) {
    println!("{n:?}")
}

#[derive(Debug)]
enum Either {
    Left(usize),
    Right(String)
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    println!("{:?}", plus_one(five));

    let none = plus_one(None);
    println!("{none:?}");

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let opt: Option<String> = Some(String::from("Hello world!"));

    match &opt {
        Some(s) => println!("Some: {s:?}"),
        None => println!("None!"),
    }

    println!("{:?}", opt);

    let x = Either::Right(String::from("Hello world"));
    println!("{x:?}");
}
