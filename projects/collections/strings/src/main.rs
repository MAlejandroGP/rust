fn main() {
    println!("Hello, world!");
}

let mut s = String::new();

let data = "inital contents";

let s = data.to_string();

//The method also works on a literal directly:
let s = "initial contents".to_string();

let s = String::from("initial contents");

//strings are UTF-8 encoded, they allow a wide variety of characters
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שלום");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");

let mut s = String::from("foo");
s.push_str("bar");

let mut s1 = String::from("foo");
let s2 = "bar";
//push_str takes a slice and not ownership
s1.push_str(s2);
println!("s2 is {s2}");

let mut s = String::from("lo");
s.push('l');

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

//This is complicated
let s = s1 + "-" + &s2 + "-" + &s3;

//This is simpler
//It also doesn't take ownership of any of the parameters
let s = format!("{s1}-{s2}-{s3}");


let s1 = String::from("hi");
//We can't index Strings
let h = s1[0];

//A String is a wrapper over a Vec<u8>
let hello = String::from("Hola");

let hello = "Здравствуйте";

//This slice is out of range
//Creating slices with ranges can crash the program
let s = &hello[0..4];


//Iterating over strings
for c in "Зд".chars() {
    println!("{c}");
}

for b in "Зд".bytes() {
    println!("{b}");
}