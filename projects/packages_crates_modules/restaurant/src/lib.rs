mod front_of_house {
    //This allows the module to be accessed from outside
    pub mod hosting {
        //Making the module public doesn't make its contents public
        //We must declare a function public to be able to access it
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    //All of the enum's variants are public because the enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//Re-exporting the code to make it available to outside code
use crate::front_of_house::hosting;

mod customer {  
    //super::hosting would make it work again
    pub fn eat_at_restaurant() {
        //This won't compile since it is at a different scope than the use statement for hosting
        //The compiler will search for hosting at customer module
        hosting::add_to_waitlist();
    }
}