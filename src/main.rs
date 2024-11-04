use std::env;
use std::io;
use std::process;

enum Pattern {
    Single(String),
    Digit,
    Character,
    CharacterGroup(String),
    NegetiveCharacterGroup(String),
    Unknown,
}

impl From<String> for Pattern {
    fn from(string_pattern: String) -> Pattern {
        if string_pattern.chars().count() == 1 {
            return Pattern::Single(string_pattern);
        } else if string_pattern == "\\d" {
            return Pattern::Digit;
        } else if string_pattern == "\\w" {
            return Pattern::Character;
        } else if string_pattern.starts_with("[") & string_pattern.ends_with("]") {
            if let Some('^') = string_pattern.chars().nth(1) {
                let actual_pattern = string_pattern
                    .to_string()
                    .trim_matches('[')
                    .trim_matches('^')
                    .trim_matches(']')
                    .to_string();
                return Pattern::NegetiveCharacterGroup(actual_pattern);
            } else {
                let actual_pattern = string_pattern
                    .to_string()
                    .trim_matches('[')
                    .trim_matches(']')
                    .to_string();
                return Pattern::CharacterGroup(actual_pattern);
            }
        } else {
            return Pattern::Unknown;
        }
    }
}

fn match_single_letter(input_line: &str, pattern: char) -> bool {
    println!("checking for a single character match");
    input_line.contains(pattern)
}

fn match_digit(input_line: &str) -> bool {
    println!("checking for a digit match");
    input_line.contains(|c: char| c.is_digit(10))
}

fn match_character(input_line: &str) -> bool {
    println!("checking for a character match");
    input_line.contains(|c: char| c.is_ascii_alphanumeric())
}

fn match_character_group(input_line: &str, pattern: &str) -> bool {
    println!("checking for a character group match");
    input_line.contains(|c: char| match_single_letter(pattern, c))
}

fn match_negetive_character_group(input_line: &str, pattern: &str) -> bool {
    println!("checking for a negetive character group match");
    !input_line.contains(|c: char| match_single_letter(pattern, c))
}

fn match_pattern(input_line: &str, pattern: Pattern) -> bool {
    return match pattern {
        Pattern::Single(s) => match_single_letter(input_line, s.chars().next().unwrap()),
        Pattern::Digit => match_digit(input_line),
        Pattern::Character => match_character(input_line),
        Pattern::CharacterGroup(s) => match_character_group(input_line, &s),
        Pattern::NegetiveCharacterGroup(s) => match_negetive_character_group(input_line, &s),
        Pattern::Unknown => false,
    };
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let string_pattern = env::args().nth(2).unwrap();
    let pattern: Pattern = Pattern::from(string_pattern);

    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, pattern) {
        println!("True");
        process::exit(0)
    } else {
        println!("False");
        process::exit(1)
    }
}
