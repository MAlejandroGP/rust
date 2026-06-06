fn main() {

}

fn use_first_word(){
    let mut s = String::from("hello world");

    let word = first_word(&s); //word will get the value 5

    s.clear();//this empties the String, making it equal to ""

    //word still has the value 5 here, 
    //but s no longer has any content that we could meaningfully use with the value 5, 
    //so word is now totally invalid!
}

fn use_first_word_slice(){
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}

fn slice(){
    let s = String::from("hello world");

    //Reference to first portion of the String
    let hello = &s[0..5];
    //Reference to the second portion of the String
    let world = &s[6..11];
}

fn slice_format(){
    let s = String::from("hello");

    //These are the same
    let slice = &s[0..2];
    let slice = &s[..2];
}

fn slice_format_2(){
    let s = String::from("hello");

    let len = s.len();

    //These are the same too
    let slice = &s[3..len];
    let slice = &s[3..];
}

fn slice_format_3(){
    let s = String::from("hello");

    let len = s.len();

    //These too
    let slice = &s[0..len];
    let slice = &s[..];
}

fn first_word_slice(){
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}

fn other_slices(){
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}