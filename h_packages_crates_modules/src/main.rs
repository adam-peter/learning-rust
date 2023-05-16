fn main() {
    let test = "test";
    println!("{}", remove_char(test));
    println!(":D");
}

fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}
