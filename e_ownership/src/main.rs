// use std::rc::Rc;

// fn main() {
//     //Returning reference to the stack from a function
//     println!("{}\n", move_ownership_out());
//     println!("{}\n", return_static_string_literal());
//     println!("{}\n", using_garbage_collection());

//     let mut my_string = String::from("Adam Peter");
//     println!("{}", my_string);
//     mutate_a_string(&mut my_string);
//     println!("{}\n", my_string);

//     //Not enough permissions
//     let mut name: Vec<String> = vec![String::from("Adam"), String::from("Peter")];
//     let name_with_title = add_title_to_name(&mut name);
//     println!("My full name is {}", name_with_title);
//     println!("Mutated vector :c {:?}\n", name);

//     let name: Vec<String> = vec![String::from("Adam"), String::from("Peter")];
//     let name_with_title = add_title_without_mutation(&name);
//     println!("My full name is {}", name_with_title);
//     println!("Vector stays the same B) {:?}\n", name);

//     let name: Vec<String> = vec![String::from("Adam"), String::from("Peter")];
//     let name_with_title = add_title_without_cloning(&name);
//     println!("My full name is {}", name_with_title);
//     println!("...blazingly fast {:?}", name);

//     //Copying x Moving out of a collections
//     let v: Vec<i32> = vec![0, 1, 2];
//     let mut n = v[0];
//     n = 69;
//     println!("{}", n); //all good
//     println!("{:?}\n", v); //all good

//     let str_v: Vec<String> = vec![String::from("Hello")];
//     //let s = str_v[0] -> we get ownership -> might cause undefined behavior
//     //1. get the reference
//     let s: &String = &str_v[0];
//     println!("{}\n", s);

//     //2. clone the data
//     let mut s: String = str_v[0].clone();
//     s.replace_range(.., "world!");
//     println!("String: {}; Vector: {:?}\n", s, str_v);

//     //3. remove the string from the vector
//     let mut str_v: Vec<String> = vec![String::from("Hello")];
//     let mut s = str_v.remove(0);
//     s.replace_range(.., "world!");
//     println!("String: {}; Vector: {:?}\n", s, str_v);

// }

// fn move_ownership_out() -> String {
//     let s = String::from("Hello world!");
//     s
// }

// fn return_static_string_literal() -> &'static str {
//     "Hello world!"
// }

// fn using_garbage_collection() -> Rc<String> {
//     //defers lifetime-checking to runtime, uses garbage collection
//     let s = Rc::new(String::from("Hello world!"));
//     Rc::clone(&s)
// }

// fn mutate_a_string(to_mutate: &mut String) {
//     to_mutate.replace_range(.., "Hello world!");
// }

// fn add_title_to_name(name: &mut Vec<String>) -> String {
//     //this also mutates the original vector..
//     name.push(String::from("PHD."));
//     name.join(" ")
// }

// fn add_title_without_mutation(name: &Vec<String>) -> String {
//     let mut name_clone = name.clone();
//     name_clone.push(String::from("PHD."));
//     name_clone.join(" ")
// }

// fn add_title_without_cloning(name: &Vec<String>) -> String {
//     let mut full_name = name.join(" ");
//     full_name.push_str(" PHD.");
//     full_name
// }

fn main(){
    let words = String::from("Hello world!");
    let first_word = return_first(&words);
    println!("First word: {}", first_word);

    let my_arr = [1, 2, 3, 4, 5];
    let my_arr_slice = &my_arr[1..3];
    assert_eq!(my_arr_slice, &[2, 3]); //all gucchi
}

fn return_first(words: &str) -> &str{
    let bytes = words.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &words[..i];
        }
    }
    words
}

