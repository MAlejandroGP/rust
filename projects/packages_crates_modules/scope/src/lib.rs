use std::fmt;
use std::io;

//Two Result types with different parent modules
fn function1() -> fmt::Result {
    // --snip--
}

//In this case, skipping the parent module is not allowed
fn function2() -> io::Result<()> {
    // --snip--
}

//This helps distinguish between the two

//Could alse use as to give one a different name
//use std::fmt::Result;
//use std::io::Result as IoResult;