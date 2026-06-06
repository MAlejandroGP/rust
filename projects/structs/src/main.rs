fn main() {
    println!("Hello, world!");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn user(){
    let user1 = User{
                    active: true,
                    username: String::from("someusername123"),
                    email: String::from("someone@example.com"),
                    sign_in_count: 1,
                };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User{
        email:String::from("another@example.com"), //We specify one value
        ..user1 //This fills the rest of the fields with this instance's values
    };
}

fn build_user(email: String, username: String) -> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

