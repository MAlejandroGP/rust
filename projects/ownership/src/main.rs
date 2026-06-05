fn main() {
    println!("Hello, world!");
}

fn scope(){         //s is not valid here, since it's not yet declared
    let s = "hello";//s is valid from this point forward

    //do stuff with s
}                   //this scrope is now over, and s is no longer valid

fn string_from(){
    //creating String from a string literal
    let s = String::from("hello");
}

fn mutable_string(){
    //Requesting memory from the allocator at runtime
    let mut s = String::from("hello");

    s.push_str(", world!"); //push_str() appends a literal to a String

    println!("{s}"); //this will print `hello, world!`
} //The memory is returned calling drop

fn data_interaction(){
    //Multiple variables can interact with the same data
    let x = 5;
    //Copy the value of x and bind it to y
    let y = x;
}

fn data_interaction_pointer(){
    let s1 = String::from("hello");
    //Copy the pointer, the length and the capacity on the stack
    //The data on the heap is not copied
    let s2 = s1;
    //If the data on the heap was copied, 
    //the operation could be very expensive
}

fn data_interaction_invalid_reference(){
    let s1 = String::from("hello");

    let s2 = s1; //s1 is no longer valid after this
    //s1 has been moved to s2

    //This doesn't compile because it is trying to use an invalidated reference
    println!("{s1}, world!");
}

fn scope_and_assignment(){
    let mut s = String::from("hello");
    //Previous value is immediately freed from memory
    s = String::from("ahoy");

    println!("{s}, world!");
}

fn variables_and_data_interacting_with_clone(){
    let s1 = String::from("hello");
    let s2 = s1.clone(); //Copying heap data

    println!("s1 = {s1}, s2 = {s2}");
}

fn stack_only_data_copy(){
    let x = 5;
    let y = x;
    //Types that have a known size at compile time are stored on the stack
    //Copying values in the stack is quick so it is allowed
    //Any type that implements the Copy trait has this behaviour

    println!("x = {x}, y = {y}");
}