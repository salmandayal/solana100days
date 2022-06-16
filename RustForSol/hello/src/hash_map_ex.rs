//HashMap is structure which stores data in key-value pair.
//It is similar to dictionary in other languages.
//It needed to import from standard library
//Map works with references

//Map's get method returns Option<&V>
//None, to indicate or failure of value
//Some(value), a tuple struct a to indicate success of value with Type T
//To get option value we can the match approach or "unwrap" approach
//Unwrap is discourage it may panic if option returned None

//Option Example
pub fn _divide(x: i32, y: i32) -> Option<i32> {
    if x % y != 0 {
        None
    } else {
        Some(x / y)
    }
}

pub fn option_example() {
    let divide1: Option<i32> = _divide(4, 2);
    let divide2: Option<i32> = _divide(4, 3);

    //UnWrap Way
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    //Pattern Matching
    match divide2 {
        Some(x) => println!("{:?} unwraps to {}", divide2, x),
        None => println!("{:?} unwraps to {}", divide2, "None"),
    }
}

use std::collections::HashMap;
pub fn hash_map_example() {
    let mut sample_map = HashMap::new();

    sample_map.insert(0, "Hi1");
    sample_map.insert(1, "Hi2");

    println!("{:?}", sample_map);

    match sample_map.get(&0) {
        Some(str) => println!("{}", str),
        None => println!("Not found"),
    }

    match sample_map.get(&2) {
        Some(str) => println!("{}", str),
        None => println!("Not found"),
    }

    sample_map.remove(&0);
    println!("{:?}", sample_map);
}
