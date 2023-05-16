enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let my_coin = Coin::Dime;
    println!("{}", value_in_cents(&my_coin));

    if let Coin::Dime = my_coin {
        println!("It was a coin!");
    }
}


fn value_in_cents(c: &Coin) -> i32 {
    //match expression
    //match arm: pattern => some_code, (separated by commas)
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
