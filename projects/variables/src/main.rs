use std::io;

fn main() {
    
}

fn character(){
    let c = 'z';
    let z: char = 'Z'; //with explicit type notation
    let heart_eyed_cat = '😻';
}

fn boolean(){
    let t = true;

    let f: bool = false; //with explicit type annotation
}

fn numeric_operations(){
    //addition
    let sum = 5 + 10;

    //substraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //Results in -1

    //remainder
    let remainder = 43 % 5;
}

fn floating_point(){
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn shadowing(){
    let x = 5;

    //First shadowing
    let x = x + 1;

    {
        //Second shadowing
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn tuple(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn tuple_destructuring(){
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

fn tuple_destructuring_direct(){
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

fn array(){
    let a = [1, 2, 3, 4, 5];
}

fn array_months(){
    let months = ["January", "February", "March", "April", "May", "June", 
        "July", "August", "September", "October", "November", "December"];
}

fn array_typed_sized(){
    //Initialize an array with type i32 and size 5
    let a: [i32, 5] = [1, 2, 3, 4, 5];
}

fn array_sized_preinitialized(){
    //Initialize an array with a fixed sized and filled with a value
    let a = [3; 5];
}

fn array_access(){
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

fn invalid_array_access(){
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    //This allows index to have an out of bounds value
    //If index is out of bounds, the program will panic
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}