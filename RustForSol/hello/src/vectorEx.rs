pub fn vector_example(){
    let mut v:Vec<i32> = vec![1,2,3,4,5];
    v.push(6);
    v.len();
    v.remove(5);
    println!("Print Vec:{:?}",v);
}