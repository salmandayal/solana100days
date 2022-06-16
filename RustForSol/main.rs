fn main() {
    match_example(2);
}

pub fn match_example(x:u8) {
    match x {
        0 => println!("zero"),
        1..=3 => println!("1 to 3"),
        10|12 => println!("Either 10 or 12"),
        _ => println!("Running out of options")
    }
}