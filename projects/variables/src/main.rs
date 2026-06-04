fn main() {
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