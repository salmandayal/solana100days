//Results are like Option<T> but with error handling
//Returns Ok(value) or Err

//We can also use expect function to indicate the error

#[derive(Debug)]
enum MyError {
    DivideError,
}

fn divide(divisor: i32, dividend: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::DivideError)
    } else {
        Ok(dividend / divisor)
    }
}

pub fn main() {
    let divide_result = divide(2, 3);
    let res = divide_result.expect("Crash");
    // let mut divide_value: i32 = 0;
    //One way to check our Result
    // match divide_result {
    //     Ok(x) => println!("{}", x),
    //     Err(e) => println!("Error {:?}", e),
    // }
    // if divide_result.is_ok() {
    //     println!("ok");
    //     divide_value = divide_result.unwrap()
    // }
    // println!("{}", divide_value);
    //--We can use is_ok to check if Result is Ok
}
