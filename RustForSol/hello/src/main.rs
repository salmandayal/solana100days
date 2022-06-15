fn main() {
    let arr: [i8; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[1..0];
    println!("{:?}", slice);
}
//20:48
