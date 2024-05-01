fn main() {
    let bytes: [i32; 5] = [0; 5];
    let chars: [&str; 27] = [" "; 27];
    let week_days: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    let byte = bytes[1];
    let first_day = week_days[0];
    let last_day = week_days[week_days.len() - 1];

    println!("First day of the week is {}", first_day);
    println!("Last day of the week is {}", last_day);
    println!("First byte is {}", byte);
    println!("First char is {}", chars[0]);
}
