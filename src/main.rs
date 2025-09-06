use std::env;
use std::io;
use std::process;

trait WordClass {
    fn is_alphanumeric_and_underscore(&self) -> bool;
}
impl WordClass for char {
    fn is_alphanumeric_and_underscore(&self) -> bool {
        self.is_ascii_alphanumeric() || self == &'_'
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    eprintln!("Pattern: {}", pattern);
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern == r"\d" {
        input_line.chars().any(|c| c.is_ascii_digit())
    } else if pattern == r"\w" {
        input_line
            .chars()
            .any(|c| c.is_alphanumeric_and_underscore())
    } else if pattern.starts_with('[') {
        if pattern
            .chars()
            .nth(1)
            .expect("Char at index 1 error while parsing ^")
            == '^'
        {
            println!("We are inside the negative group");

            let inner = pattern.trim_start_matches("[^").trim_end_matches(']');
            println!("Inner: {}", inner);
            input_line.chars().any(|c| !inner.contains(c));
            // inner.chars().all(|c| )

            return true;
        } else {
            let inner = pattern.trim_start_matches('[').trim_end_matches(']');
            inner.chars().any(|c| input_line.contains(c))
        }
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args()
        .nth(1)
        .expect("Expected first argument to be '-E'")
        != "-E"
    {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).expect("Pattern not passed in properly");
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Input line reading failed");

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        eprintln!("Match Pattern Called: process::exit(0)");
        process::exit(0)
    } else {
        eprintln!("Match Pattern Called: process::exit(1)");
        process::exit(1)
    }
}
