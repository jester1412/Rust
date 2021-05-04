#![allow(unused)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    //let coin = Coin::Penny; // --> set coin to be enum Coin Penny
    let coin = Coin::Quarter(UsState::Alabama); // --> set coin to be enum Coin Quarter and set enum UsState Alabama
    let mut count = 0;
    // Matching Flow
    // If coin is enum Quarter print name of state
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
    // if coin is not enum Quarter add 1 to count
        count += 1;
    }
    println!("{}",count);
}
