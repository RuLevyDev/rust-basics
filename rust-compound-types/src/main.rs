#[derive(PartialEq, Debug)]
// Declare User struct to describe vehicle with four named fields
/// User with tupla
struct User {
    name: String,
    permises: Permises,
    logged: bool,
    lvl: (Level, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for User Permises type
enum Permises {
    _Write,
    Read,
    ReadWrite,
}

#[derive(PartialEq, Debug)]
// Declare enum for User Level
enum Level {
    Junior,
    _Middle,
    Senior,
}

fn user_permises(user: &User) -> String {
    match user.permises {
        Permises::Read => String::from("Read"),
        Permises::_Write => String::from("Write"),
        Permises::ReadWrite => String::from("ReadWrite"),
    }
}
//Create a tuple for the user level and a number
//Return the user level as a string
fn user_level_number(userlvl: &(Level, u32)) -> (String, u32) {
    match userlvl {
        (Level::Junior, 10) => (String::from("Junior"), 10),
        (Level::_Middle, 20) => (String::from("Middle"), 20),
        (Level::Senior, 30) => (String::from("Senior"), 30),
        _ => (String::from("Unknown"), 0),
    }
}
fn set_user_level(userlvl: &mut (Level, u32), level: Level, number: u32) {
    *userlvl = (level, number);
}
fn set_user_permises(user: &mut User, permises: Permises) {
    user.permises = permises;
}

fn user_level(userlvl: &Level) -> String {
    match userlvl {
        Level::Junior => String::from("Junior"),
        Level::_Middle => String::from("Middle"),
        Level::Senior => String::from("Senior"),
    }
}

/// main function
/// User para que tenga un campo de tupla
fn main() {
    let mut user = User {
        name: String::from("Red"),
        permises: Permises::Read,
        logged: true,
        lvl: (Level::Senior, 100),
    };

    println!("{:?}", user);
    println!("User name: {:?}", user.name);
    println!("User logged: {:?}", user.logged);

    let level = Level::Junior;
    let number = 10;
    //&mut: Esto indica que estás tomando una referencia mutable del valor al que estás apuntando. Es decir, estás creando una referencia que permite modificar el valor al que apunta.
    set_user_level(&mut user.lvl, level, number);
    println!("{:?}", user);

    set_user_level(&mut user.lvl, Level::_Middle, 50);
    println!("{:?}", user);

    set_user_permises(&mut user, Permises::ReadWrite);

    println!("User level number: {:?}", user_level_number(&user.lvl));
    println!("User level: {:?}", user_level(&user.lvl.0));
    println!("User permises: {:?}", user_permises(&user));

    println!("{:?}", user);
}
