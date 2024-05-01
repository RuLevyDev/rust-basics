fn main() {
    println!("Hello, world!");
    goodbye();
    send_message("you have a message!");
    let result = divide(10, 0);
    println!("Result: {}", result);
    divide(2, 3);
}

fn goodbye() {
    println!("Goodbye, world!");
}

fn send_message(message: &str) {
    println!("Message: {}", message);
}
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero!");
    }
    a / b
}
