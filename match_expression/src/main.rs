#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
    _Arizona,
    _Arkansas,
    _California,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // Match allows us to compare a value against a series of patterns and then execute code based on which pattern matches.
    // The match is exhaustive, which means that we must exhaust every last possibility in order for the code to be valid.
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // We can use the variable `state` to match against the `UsState` enum
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("The value of the coin is: {}", value_in_cents(coin));
    // The value of the coin is: 25
}
