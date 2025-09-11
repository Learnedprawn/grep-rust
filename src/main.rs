use std::env;
use std::io;
use std::process;

enum SubPattern {
    Character(char),
    WordClass,
    DigitClass,
    PositiveGroup(Vec<SubPattern>),
    NegativeGroup(Vec<SubPattern>),
}

struct Pattern {
    pattern: Vec<SubPattern>,
}

fn parse_one_subpattern(s: &str) -> Result<Option<SubPattern>, String> {
    let mut s_iter = s.chars();

    let sub_pattern = match s_iter.next() {
        Some('\\') => match s_iter.next() {
            Some('d') => SubPattern::DigitClass,
            Some('w') => SubPattern::WordClass,
            Some('\\') => SubPattern::Character('\\'),
            Some(c) => return Err(format!("Unknown class used: \\{c}")),
            None => return Err(("Expected one more character after \\").into()),
        },
        Some('[') => {
            let mut inner = Vec::new();
            let negative = s_iter.clone().next() == Some('^');
            if negative {
                s_iter.next();
            }

            loop {
                match s_iter.next() {
                    Some(']') if negative => break SubPattern::NegativeGroup(inner),
                    Some(']') => break SubPattern::PositiveGroup(inner),
                    Some(c) => inner.push(SubPattern::Character(c)),
                    None => return Err("Matching ] Not found".into()),
                }
            }
        }
        Some(c) => SubPattern::Character(c),
        None => return Ok(None),
    };
    Ok(Some(sub_pattern))
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// fn match_pattern_charwise()

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
