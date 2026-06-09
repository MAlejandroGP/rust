fn main() {
    
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

//All functions within an impl block are called associated functions
impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //You can implement methods with the same name as the struct's fields
    fn width(&self) -> bool {
        self.width > 0
    }

    //Methods can have multiple parameters
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn methods(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    //Call to method
    if rect1.width(){
                                                                //Call to field
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

fn methods_with_more_parameters(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

//You can have multiple impl blocks for one struct
impl Rectangle{
    //Associated functions that aren't methods are often used for constructors
    //They are often called new, but this isn't mandatory and new is not a reserved word
    fn square(size: u32) -> Self{
        Self{
            width: size,
            height: size,
        }
    }

    //To call this associated function we use ::
    //e.g. let sq = Rectangle::square(3);
}