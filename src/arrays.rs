use core::num;

//Arrays - Fixed list where elements are the same data type
//Vector- Growable arrays
//Arrays are stack allocate

use std::mem;

pub fn run() {
    let mut numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];

    //Get single value
    println!("Single:{}", numbers[0]);

    //Reassign value
    numbers[2] = 10;
    println!("3rd Index:{}", numbers[2]);

    println!("{:?}", numbers);

    //& - Reference ( When we want to pass the reference to the variable)
    println!("Array occupies:{}", mem::size_of_val(&numbers));

    //My assumption to save a reference  we also define type with &
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
