pub fn run() {
    //Basic Print
    println!("Hello, world!");

    //Basic Formatting
    println!("{} is from {}", "Dayal", "Dayala");

    //Positional Arguments
    println!(
        "{2} is a {0}, works as {1}",
        "Developer", "Software Engineer", "Dayal"
    );

    //Name Arguments
    println!(
        "{name} like to play {activity}",
        name = "Dayal",
        activity = "Cricket"
    );

    //Place Holder Syntax
    println!("Binary :{:b  }, hex:{:x} and octal:{:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
