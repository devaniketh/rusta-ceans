use std::io::{self, Write};

fn main() {
    print!("Enter a string to reverse: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input = input.trim().to_string();

    let reversed = reverse_string(&input);

    println!("Reversed string: {}", reversed);
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}