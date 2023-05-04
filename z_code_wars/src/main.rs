fn main() {
    let phrase = "world";
    println!("{}", solution(phrase));
}

fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}
