struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // create new user:
    let mut user1 = User {
        active: true,
        username: String::from("some name"),
        email: String::from("some_name@gmail.com")
        sign_in_count: 1,
    };
    //change value of user email - object must be mutable
    user1.email = String::from("new_email@gmail.com");
    
    

}
