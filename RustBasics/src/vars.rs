//Var are primtive to data or referenece to data
//Var are immutable by default and mutable if declared mut
//Rust is block scoped language
//For constant we must define type on initialization

pub fn run() {
    let name = "Dayal";
    let mut age = 23;
    println!("My name is {} and I'm  {}", name, age);
    age = 24;
    println!("My name is {} and I'm  {}", name, age);
    //Define constant
    const ID: i32 = 001;
    println!("My ID: {}", ID);

    //Assign multiple variables
    let (my_name, my_age) = ("Salman", "26");
    print!("My name is {} and I am {}", my_name, my_age);
    //Continue 24:45
}
