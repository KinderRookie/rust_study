use crate::Coins::Penny;

fn main() {

    value_in_cents_state(StateCoin::Quarter(UsState::California));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice = 4;

    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => println!("{:?}", other), // 포괄적 match

    }

    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => println!("\n"), // value 바인딩이 필요 없는 포괄적 match
    }


}

enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter => 25,
    }
}

// 값을 바인딩하는 패턴
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
}

enum StateCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_state(coin: StateCoin) -> u8 {
    match coin {
        StateCoin::Penny => 1,
        StateCoin::Nickel => 5,
        StateCoin::Dime => 10,
        StateCoin::Quarter(state) => {
            println!("State quarter from {:?}", state); // -> 값이 바인딩 된다.
            25
        }
    }
}

// Option<T> matching
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        // Some 내부에 담긴 값은 i 에 바인딩 된다.
    }
}



fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
