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


fn ownership_and_functions(){
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn return_values_and_scope(){
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn return_ownership_of_parameters(){
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}