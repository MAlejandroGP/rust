fn main() {
    println!("Hello, world!");
}

enum Coin {
    Penny, 
    Nickel, 
    Dime, 
    Quarter, 
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_multiline(coin: Coin) -> u8 {
    match coin{
        Coin::Penny =>{
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]//so we can inspect the state
enum UsState{
    Alabama, 
    Alaska, 
    American 
    Samoa, 
    Arizona, 
    Arkansas, 
    California, 
    Colorado, 
    Connecticut, 
    Delaware, 
    DistrictOfColumbia, 
    Florida, 
    Georgia, 
    Guam, 
    Hawaii, 
    Idaho, 
    Illinois, 
    Indiana, 
    Iowa, 
    Kansas, 
    Kentucky, 
    Louisiana, 
    Maine, 
    Maryland, 
    Massachusetts, 
    Michigan, 
    Minnesota, 
    Mississippi, 
    Missouri, 
    Montana, 
    Nebraska, 
    Nevada, 
    NewHampshire, 
    NewJersey, 
    NewMexico, 
    NewYork, 
    NorthCarolina, 
    NorthDakota, 
    NorthernMariana 
    Islands, 
    Ohio, 
    Oklahoma, 
    Oregon, 
    Pennsylvania, 
    Puerto Rico, 
    RhodeIsland, 
    SouthCarolina, 
    SouthDakota, 
    Tennessee, 
    Texas,  
    Virgin 
    Islands, 
    Utah, 
    Vermont, 
    Virginia, 
    Washington, 
    West 
    Virginia, 
    Wisconsin, 
    Wyoming
}

enum CoinState{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_state(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }   
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
         
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn use_plus_one(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one_error(x: Option<i32>) -> Option<i32> {
    //If every case is not evaluated, the code won't compile
    match x {
        Some(i) => Some(i + 1),
    }
}


fn dice_roll(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn dice_reroll(){
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //Wildcard pattern
        _ => reroll(),
    }
}

fn reroll() {}

fn dice_nothing(){
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //If it's not 3 or 7, nothing happens
        _ => (),
    }
}