//Vectors - Growable Arrays / Resizeable arrays

use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2];
    let newVec: Vec<i64> = vec![1, 2, 3, 4, 5];
    ass

    //Add on to vector
    numbers.push(3);
    println!("{:?}", numbers);

    println!("Size of vector: {}", mem::size_of_val(&numbers));

    println!("Lenth of vector:{}", numbers.len());

    let vecSlice: &[i32] = &numbers[2..3];
    println!("{:?}", vecSlice);
    for number in numbers.iter() {
        println!("{}", number);
    }
}
