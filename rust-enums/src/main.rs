fn main() {
    println!("Hello, world!");
    print!("user type: {:?}\n", UserType::Admin);
    //create a user
    let _user1 = User {
        username: String::from("user1"),
        email: String::from("sdokokjsdf"),
        sign_in_count: 1,
        active: true,
        user_type: UserType::Admin,
    };
    print!("user1: {:?}\n", _user1);
    //create a admin
    let _admin = build_user(
        String::from("Admin"),
        String::from("admin"),
        UserType::Admin,
    );
    //create a coach

    let _coach = build_user(
        String::from("Coach"),
        String::from("coach"),
        UserType::Coach,
    );
    //create a pro
    let _pro = build_user(String::from("Pro"), String::from("pro"), UserType::Pro);
    //create a player
    let _player = build_user(
        String::from("Player"),
        String::from("player"),
        UserType::Player,
    );
    //create a guest
    let _guest = build_guest(String::from("Guest"), String::from("guest"));
}

//define a struct for user
#[derive(Debug, PartialEq)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    user_type: UserType,
}
#[derive(PartialEq, Debug)]
//delare enums for user type
enum UserType {
    Admin,
    Coach,
    Pro,
    Player,
    Guest,
}
//function to build a user geust
fn build_guest(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
        user_type: UserType::Guest,
    }
}
//function to build a user with the user type
fn build_user(email: String, username: String, user_type: UserType) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
        user_type,
    }
}
