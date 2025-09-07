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
    } else if pattern == "\\d" {
        input_line.chars().any(|c| c.is_ascii_digit())
    } else if pattern == "\\w" {
        println!(
            "Input_line: {}, Output: {}",
            input_line,
            input_line
                .chars()
                .any(|c| c.is_alphanumeric_and_underscore())
        );
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
            let inner = pattern.trim_start_matches("[^").trim_end_matches(']');
            input_line.chars().any(|c| !inner.contains(c))
        } else {
            let inner = pattern.trim_start_matches('[').trim_end_matches(']');
            inner.chars().any(|c| input_line.contains(c))
        }
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

fn match_pattern_charwise(input_line: &str, pattern: &str) -> bool {
    let mut input_iter = input_line.chars();
    let mut pattern_iter = pattern.chars();

    loop {
        match (input_iter.next(), pattern_iter.next()) {
            (Some(input_char), Some(pattern_char)) => {
                println!("input_char: {}, pattern_char: {}", input_char, pattern_char);
                if pattern_char == '\\' {
                    println!("\\ matched");
                    let class = pattern_iter.next().unwrap();
                    println!("class: {}", class);
                    // if pattern_iter.next().expect("Value after \\ weird") == 'd' {
                    if class == 'd' {
                        if !match_pattern(input_char.to_string().as_str(), "\\d") {
                            println!("This is being called: d");
                            return false;
                        }
                    }
                    // if pattern_iter.next().expect("Value after \\ weird") == 'w' {
                    if class == 'w' {
                        if !match_pattern(input_char.to_string().as_str(), "\\w") {
                            println!("This is being called: w");
                            return false;
                        }
                    }
                } else if pattern_char == '[' {
                    let char_after_square_bracket = pattern_iter
                        .clone()
                        .next()
                        .expect("Square bracket next error");
                    if char_after_square_bracket == '^' {
                        pattern_iter.next().unwrap();
                        let mut group = String::new();
                        loop {
                            match pattern_iter.next() {
                                Some(']') => {
                                    break;
                                }
                                Some(char) => {
                                    group.push(char);
                                }
                                None => panic!("No matching ] found"),
                            }
                        }
                        println!("Output: {}", !group.chars().any(|c| input_line.contains(c)));
                        return !group.chars().any(|c| input_line.contains(c));
                    } else {
                        let mut group = String::new();
                        loop {
                            match pattern_iter.next() {
                                Some(']') => {
                                    break;
                                }
                                Some(char) => {
                                    group.push(char);
                                }
                                None => panic!("No matching ] found"),
                            }
                        }
                        group.contains(input_char);
                        return group.chars().any(|c| input_line.contains(c));
                    }
                } else {
                    if pattern_char != input_char {
                        println!("\\ not matched");
                        println!("pattern_char: {}, input_char: {}", pattern_char, input_char);
                        return false;
                    }
                }
            }
            (None, None) => return true,
            (Some(input_char), None) => {
                println!("pattern_char is none and input_char: {}", input_char);
                return false;
            }
            (None, Some(pattern_char)) => {
                println!("input_char is none and pattern_char: {}", pattern_char);
                return false;
            } // _ => {
              //     println!("This is being called: _");
              //     return false;
              // }
        }
    }
}

fn pattern_len(pattern: &str) -> usize {
    let mut length = 0;
    let pattern = pattern.trim_start_matches('[').trim_end_matches(']');
    let mut pattern_iter = pattern.chars();

    while let Some(char) = pattern_iter.next() {
        if char == '\\' {
            pattern_iter.next();
        }
        length += 1;
    }
    length
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
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

    // Newline Removal
    let input_line = input_line.trim();

    let pattern_length;
    if pattern.chars().nth(0).unwrap() == '[' {
        pattern_length = input_line.len();
    } else {
        pattern_length = pattern_len(&pattern);
    }

    println!("pattern: {}, len: {}", pattern, pattern_length);

    println!("input_line: {}, len: {}", input_line, input_line.len());

    for i in 0..=(input_line.len() - pattern_length) {
        let window = &input_line[i..i + pattern_length];
        println!("Window: {}", window);
        if match_pattern_charwise(&window, &pattern) {
            eprintln!("Match Pattern Called: process::exit(0)");
            process::exit(0)
        } else {
            eprintln!("Did not match");
        }
    }
    eprintln!("Match Pattern Called: process::exit(1)");
    process::exit(1)

    // if match_pattern_charwise(&input_line, &pattern) {
    //     eprintln!("Match Pattern Called: process::exit(0)");
    //     process::exit(0)
    // } else {
    //     eprintln!("Match Pattern Called: process::exit(1)");
    //     process::exit(1)
    // }
}
