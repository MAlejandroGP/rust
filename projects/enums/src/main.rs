fn main() {
    println!("Hello, world!");
}

enum IpAddrKind{
    V4,
    V6,
}

fn enum_values(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

fn route(ip_kind: IpAddrKind){}

fn use_route(){
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

//An enum value can be stored in a struct
fn ip_addr(){
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

//We can store data inside an enum variant
enum IpAddrString{
    V4(String),
    V6(String),
}

//And attach data to each variant directly
fn ip_addr_enum(){
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
}

//Each variant of an enum can have different types of data associated
//You can also store structs inside an enum variant
enum IpAddrDifferentData(){
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_add_enum_different_data(){
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

//Enums can store a wide variety of types
enum Message{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//^ This
//v Is the same as this
struct QuitMessage; //unit struct
struct MoveMessage{
    x: i32
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

//We can also define methods for enums
impl Message{
    fn call(&self){
        //method body would be defined here
    }
}

fn enum_method(){
    let m = Message::Write(String::from("hello"));
    m.call()
}

//Option enum
fn option_example(){
    //Present value
    let some_number = Some(5);
    let some_char = Some('e');

    //Absent value
    let absent_number: Option<i32> = None;
}

fn add_option(){
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //This throws an error
    //We can't add an Option to an integer
    let sum = x + y;
}