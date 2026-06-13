fn main() {
    println!("Hello, world!");
}

//Empty integer vector
let v: Vec<i32> = Vec::new();

//Vectors can hold any type or struct
let v = vec![1, 2, 3];

let mut v = Vec::new();

//Add elements with push
v.push(5);
v.push(6);
v.push(7);
v.push(8);

let v = vec![1, 2, 3, 4, 5];

//Access a stored value via indexing
let third: &i32 = &v[2];
println!("The third element is {third}");

//Access via get, get returns an Option
let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}


let v = vec![1, 2, 3, 4, 5];

//This causes the program to panic
let does_not_exist = &v[100];

//This returns None as a value without panicking
let does_not_exist = v.get(100);

let mut mut_v = vec![1, 2, 3, 4, 5];

//Can't mix immutable references with mutable references
let first = &mut_v[0];

//This throws a compiling error because of a mutable borrow
//If the memory was full we wouldn't be able to add a value next to the last one
//This would require to allocate new memory in another space and deallocate the old one
//This would cause first to point to deallocated memory
v.push(6);

println!("The first element is: {first}");

let iter_v = vec![100, 32, 57];

//Iterating a vector
for i in &v {
        println!("{i}");
}

let mut mut_iter_v = vec![100, 32, 57];

for i in &mut mut_iter_v {
    //Must dereference i to get its value
    *i += 50;
}

enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

{
    let v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here with its contents
