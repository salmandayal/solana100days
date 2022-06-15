pub fn _for_loop(){
    for  i in 0..10 {
            println!("Even here{}", i);
    }
}

pub fn _while_loop() {
    let mut i = 0;
    while i<10 {
        println!("Even here{}", i);
        i += 1;

    }
}

pub fn match_example(){
    let i = 4;
    match i {
        0 => println!("Zero"),
        1|2|3=> println!("1 or 2"),
        4..=6=> println!("between 4 and 6"),
        _ => println!("Not betweeen 0 and 6"),
    }
}