fn main() {
    let x = plus_one(five());

    println!("The value of x is: {x}");
}

fn another_function(){
    println!("Another function.");
}

fn another_function_parameter(x: i32){
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn proof_assignments_do_not_return_values(){
    //let x = (let y = 6);
}

fn expression_inside_scope(){
    let y = {
        let x = 3;
        //x + 1; This is a statement
        x + 1 //This is an expression
    };

    println!("The value of y is: {y}");
}

fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
    //x + 1; This produces an error
}