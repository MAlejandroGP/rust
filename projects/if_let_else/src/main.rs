fn main() {
    println!("Hello, world!");
}

fn match_example(){
    let config_max = Some(3u8);
    match config_max{
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}

fn if_let_example(){
    let config_max = Some(3u8);
    //This allows to execute code conditionally without evaluating every option
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

enum UsState{
    Alabama, 
    Alaska, 
    AmericanSamoa, 
    Arizona, 
    Arkansas, 
    California, 
    //etc
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn if_let_coin(){
    let mut count = 0;
    if let Coin::Quarter(state) = coin{
        println!("State quarter from {state:?}!");
    }
    else{
        count += 1;
    };
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter_more_complicated(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_simpler(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
