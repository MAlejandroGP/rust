fn main() {
    println!("Hello, world!");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn user(){
    let user1 = User{
                    active: true,
                    username: String::from("someusername123"),
                    email: String::from("someone@example.com"),
                    sign_in_count: 1,
                };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User{
        email:String::from("another@example.com"), //We specify one value
        ..user1 //This fills the rest of the fields with this instance's values
    };
}

fn build_user(email: String, username: String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs(){
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

//Useful for implementing a trait without any data being stored
struct AlwaysEqual;

fn unit_like_structs(){
    let subject = AlwaysEqual;
}

struct UserBorrowedStrings{
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn ownership_of_struct_data(){
    //No lifetime specifiers
    //This won't compile
    let user1 = UserBorrowedStrings{
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    }
}

fn non_struct_program_example(){
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

//Less clear
fn area(width: u32, height: u32) -> u32{
    width * height
}


fn tuple_program_example(){
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

//Simpler but still not clear since tuples don't name their elements
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_program_example(){

}

struct Rectangle{
    width: u32,
    height: u32,
}

fn main(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

//Accessing a struct's named properties makes it easier to know their relation and allows for less parameters
fn area(rectangle: &Rectangle) -> u32 {
    //Accessing fields of borrowed structs does not move the field values
    rectangle.width * rectangle*height
}

fn print_struct(){
    let rect1 = Rectangle{
        width: 30.
        height: 50,
    };

    //Rectangle doesn't implement Display, so it gives a compiling error
    println!("rect1 is {rect1}");
    //Rectangle doesn't implement Debug either
    println!("rect1 is {rect1:?}");
}

//This makes it implement Debug
#[derive(Debug)]
struct RectangleDebug{
    width: u32,
    height: u32,
}
/*
{rect1:#?} would print it like
rect1 is Rectangle {
    width: 30,
    height: 50,
}
instead of rect1 is Rectangle { width: 30, height: 50 }
*/

fn debug_output(){
    let scale = 2;
    let rect1 = Rectangle {
        //This will take ownership of the value and return it, it prints to stderr instead of stdout
        width: dbg!(30 * scale),
        height: 50,
    };
    
    //This is used for debugging, it prints the line and column where it's executed and the value
    dbg!(&rect1);
}