fn main() {
    if_let();
}

fn if_example(){
    //let number = 3;
    let number = 7;

    if number < 5{
        println!("condition was true");
    }
    else{
        println!("condition was false");
    }
}

fn if_fails(){
    let number = 3;

    //Comparing integer instead of bool
    // if number{
    //     println!("number was three");
    // }
}

fn if_not_zero(){
    let number = 3;

    if number != 0{
        println!("number was something other than zero");
    }
}

fn multiple_conditions(){
    let number = 6;

    if number % 4 == 0{
        println!("number is divisible by 4");
    }
    else if number % 3 == 0{
        println!("number is divisible by 3");
    }
    else if number % 2 == 0{
        println!("number is divisible by 2");
    }
    else{
        println!("number is not divisible by 4, 3 or 2");
    }
}

fn if_let(){
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn if_let_various_types(){
    let condition = true;

    //Potential values must always be of the same type
    //let number = if condition { 5 } else { "six" };

    //println!("The value of number is: {number}");
}