//Primitive str = Immutable String fixed length
// String = Growable, heap allocated data structure - Use when you need to modify or own string data
pub fn run() {
    let _hello_str = "Hello STR";
    let mut hello = String::from("Hello"); //Type String

    hello.push(' '); //Push a character to the end of the string - Char type

    //Push String
    hello.push_str("World!");

    //Get capacity

    println!("Capacity: {}", hello.capacity());
    println!("Is Empty: {}", hello.is_empty());
    println!("Contains 'World' :{}", hello.contains(" Wo"));
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("Word: {}", word);
    }

    //
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion Testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("S : {}", s);
}
