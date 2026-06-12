//When bringin structs, enums and others it's idiomatic to specify the full path
use std::collections::HashMap;
//We can bring multiple items into scope from the same crate in one line
use std::{cmp::Ordering, io};
use std::io::{self, Write};
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}