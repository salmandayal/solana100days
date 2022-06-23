pub fn enum_example() {
    let wallet_type = String::from("ERC20");
    let a: WalletEnum = WalletEnum::Index(0);
    let b: WalletEnum = WalletEnum::Info {
        wallet_type,
        balance: 10,
    };
    println!("{:?}", a);
    println!("{:?}", b);

    //In order to extract the value of ENUM
    if let WalletEnum::Index(val) = a {
        println!("extracted Index:{}", val);
    }
    if let WalletEnum::Info {
        wallet_type,
        balance,
    } = b
    {
        println!(
            "extracted Wallet Info -> Wallet Type:{} , Balance:{}",
            wallet_type, balance
        );
    }
}
#[derive(Debug)]
enum WalletEnum {
    Index(i32),
    Info { wallet_type: String, balance: i32 },
}
