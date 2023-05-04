//Copy this and command + F to continue: "You can implement:"
//https://fasterthanli.me/articles/a-half-hour-to-learn-rust

fn main() {
    let _ = 40;
    let _ = print_stuff();
    let _unused = 2;

    //tuple
    let pair = ('a', 17);
    let _ = pair.0;
    let _ = pair.1;

    let (my_char, my_int) = pair; //destructuring
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    let num = {
        let num1 = 9;
        let num2 = 10;
        num1 + num2
    };

    //dot . - for accessing fields and methods of values
    let adam = String::from("badmood111").len();

    //double-semi :: - same as dot, for Namespaces
    let smaller = std::cmp::min(3, 8);

    //"use" - to bring things from different namespaces into scope
    use std::cmp::min;
    let smaller = min(1, 11);
    //we can use curlies to import many functions / values from namespaces
    // == "globs"
    use std::cmp::{max, max_by};
    // * - wildcard - import everything from a namespace
    use std::cmp::*;

    //rust types are also namespaces - we can call methods on them
    let x = "adam".len();
    let x = str::len("adam"); //same thing

    //str is a primitive; many referene types are in scope by default
    use std::prelude::v1::*; //this gets called by default in each module
                             //Vec, String, Option, Result

    //initializing a struct:
    struct Point {
        x: f64,
        y: f64,
    }
    let p1 = Point { x: 1., y: 3. };
    let p2 = Point { y: 2., x: 4. }; //the order doesn't matter
    let p3 = Point { x: 14., ..p2 }; //..rest operator
    let Point { x, y } = p1; //struct destructuring
    let Point { x, .. } = p2;

    let n1 = Number {
        odd: true,
        value: 8,
    };
    print_number(&n1);
    let n_one = Number {
        odd: false,
        value: 1,
    };
    let n_two = Number {
        odd: true,
        value: 2,
    };
    print_specific(&n_one);
    print_specific(&n_two);
    print_specific(&n1);

    if n1.is_strictly_positive() {
        println!("Number {} is positive.", n1.value);
    }

    println!("{}", 10.is_strictly_positive());
}

fn print_stuff() -> i32 {
    println!("Hello!");
    2
}

//struct - to implement your own type
struct Number {
    odd: bool,
    value: i32,
}

//traits - multiple types can have in common
trait Signed {
    fn is_strictly_positive(&self) -> bool;
}

impl Signed for Number {
    //implementing a trait for a type;
    fn is_strictly_positive(&self) -> bool {
        self.value > 0
    }
}

impl Signed for i32 {
    //implementing our trait on a foreign type
    fn is_strictly_positive(&self) -> bool {
        *self > 0
    }
}

fn print_number(n: &Number) {
    match n {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
    }
}

fn print_specific(n: &Number) {
    //match must be exhaustive (hence the last case here)
    match n {
        Number { value: 1, .. } => println!("One!"),
        Number { value: 2, .. } => println!("Two!"),
        Number { value, .. } => println!("Number: {}", value),
        // _ => println!(":D"), - a catch-all case
    }
}