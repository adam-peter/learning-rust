fn main() {
    println!("{}", value_in_cents(&Coin::Penny));
    println!("{}", value_in_cents(&Coin::Quarter(UsState::NewYork)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //match catchall
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //exhaustive last variant, using a catchall variable (we use it in the match arm's code)
        other_number => move_player(other_number),
        //if we don't want to use the catchall value - wildcard
        //_ => (),
    }

    //careful with match and ownership!
    let name = Some(String::from("Adam"));
    match &name {
        // Some(_) => println!("Some!"), - ok (_ doesn't take ownership of the value)
        //Some(s) => println!("Some: {}", s), - error
        //Some(&s) => println!("Some: {}", s), - error (can't borrow only the String - we must borrow the whole enum!)
        Some(s) => println!("Some: {}", s),
        None => println!("None!"), //rust pushes down the reference -> s has type &String -> name can be used after match
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    //patterns can bind to values -> we can handle them with match
    Quarter(UsState),
}

fn value_in_cents(c: &Coin) -> i32 {
    //match expression
    //match arm: pattern => some_expression, (separated by commas)
    match c {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n + 1),
        None => None,
    }
}
