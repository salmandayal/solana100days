macro_rules! a_macro {
    () => {
        println!("Hello, world!");
    };
}

macro_rules! x_and_y {
    (x=>$e:expr) => {
        println!("x is {}", $e)
    };
    (y=> $e:expr) => {
        println!("y is {}", $e)
    };
}

macro_rules! build_fn {
    //Ident stands for Indentifier used to variable name anf function name
    ($func_name:ident) => {
        fn $func_name() {
            println!("This is a function named {}", stringify!($func_name));
        }
    };
}

macro_rules! exame {
    ($l:expr; and $r:expr) => {
        println!(
            "{:?} and {:?} = {:?}",
            stringify!($l),
            stringify!($r),
            $l && $r
        );
    };
    ($l:expr; or $r:expr) => {
        println!(
            "{:?} or {:?} = {:?}",
            stringify!($l),
            stringify!($r),
            $l || $r
        );
    };
}

macro_rules! compr {
    //| tell where x is
    ($id1:ident | $id2:ident <- [$start:expr;$end:expr], $cond:expr ) => {{
        let mut vec = Vec::new();
        for i in $start..$end + 1 {
            if $cond(i) {
                vec.push(i);
            }
        }
        vec
    }};
}

fn even(x: i32) -> bool {
    x % 2 == 0
}

fn odd(x: i32) -> bool {
    x % 2 != 0
}

macro_rules! new_map {
    //$()* represent that an argument can repeat 0 or more time

    // ($($key:expr => $val:expr)*) => {
    //     {
    //         let mut map = std::collections::HashMap::new();
    //         //We wrapped the operation to tell this block will repeat as many as expression repeat.
    //         $(
    //             map.insert($key, $val);
    //         )*
    //         map
    //     }
    // };
    // // , (Comma) is used to separate the expression
    // ($($key:expr => $value:expr),*) => {
    //     {
    //         let mut map = std::collections::HashMap::new();
    //         $(
    //             map.insert($key, $value);
    //         )*
    //         map
    //     }
    // };
    //If we add '+' (Plus) at the end argument in our macro, this signifies that argument pattern must appear ones
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}

macro_rules! calc {
    (eval $e:expr) => {{
        {
            let val:usize = $e;
            println!("{}", val);
        }
    }};
    (eval $e:expr, $(eval $es:expr), +) => {
        {
            calc!{eval $e}
            calc!{ $(eval $es), + }
        }
    }
}

pub fn run() {
    // x_and_y!(x=> 10);
    // x_and_y!(y=> 20);

    // build_fn!(hi_there);
    // hi_there();

    // exame!(true; and false);

    // let evens = compr![x | x <- [1;10],even];
    // let odds = compr!(x|x <- [1;10],odd);
    // print!("{:?}", odds);

    // let m = new_map! {
    //     "one" => 1
    //     "two" => 2
    //     "three" => 3
    // };
    // print!("{:?}", m);

    //Comma separated macro_rules!
    // let m = new_map! {
    //     "one" => 1
    // };
    // print!("{:?}", m);

    calc! {
        eval 1*5
    }
}
