<<<<<<< HEAD
fn main() {}
=======
use std::env;
use std::io;
use std::process;
mod tests;
fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        single_letter if single_letter.len() == 1 => input_line.contains(single_letter),
        r#"\w"# => input_line.contains(|c: char| c.is_alphanumeric()),
        r#"\d"# => input_line.contains(|c: char| c.is_digit(10)),
        pat if pat.starts_with("[^") && pat.ends_with("]") => pattern[2..pattern.len() - 1]
            .chars()
            .all(|x| !input_line.contains(x)),
        pat if pat.starts_with("[") && pat.ends_with("]") => {
            let pattern = &pattern[1..pattern.len() - 1];
            for c in pattern.chars() {
                if input_line.contains(c) {
                    return true;
                }
            }
            return false;
        }
        _ => false,
    }
}
pub fn grep(input: &str, pattern: &str) -> bool {
    let pattern: Vec<&str> = pattern.split("").filter(|x| *x != "").collect();
    let mut groups = String::new();
    let mut slash_flag = false;
    let mut brack_flag = false;
    let mut final_pat: Vec<String> = Vec::new();

    for (index, i) in pattern.iter().enumerate() {
        if i.to_string() == r#"\"# {
            slash_flag = true;
            final_pat.push(format!("\\{}", pattern[index + 1]));
            continue;
        }
        if i.to_string() == "[" {
            brack_flag = true;
        }
        if !brack_flag && !slash_flag {
            final_pat.push(i.to_string());
        } else if !slash_flag {
            groups = groups + i;
        }
        slash_flag = false;

        if i.to_string() == "]" {
            brack_flag = false;
            final_pat.push(groups);
            groups = String::from("");
        }
    }
    if match_wild_patterns(String::from(input), &mut final_pat) {
        println!("true");
        return true;
    }

    println!("false");
    return false;
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    if grep(&input_line, &pattern) {
        process::exit(0)
    }
    process::exit(1)
}
fn match_wild_patterns(inputline: String, pattern: &mut Vec<String>) -> bool {
    if &pattern[pattern.len() - 1] == "$" {
        pattern.reverse();
        let mut j = inputline.trim().len() - 1;
        for (x, i) in pattern[1..].iter().enumerate() {
            if let Some(input) = inputline.chars().nth(j) {
                if !match_pattern(&input.to_string(), i) {
                    return false;
                }
            } else {
                return false;
            }
            j -= 1;
        }
        return true;
    }
    for (index, letter) in inputline.chars().enumerate() {
        if &pattern[0] == "^" {
            for (x, i) in pattern[1..].iter().enumerate() {
                println!("{i}");
                if let Some(input) = inputline.chars().nth(x) {
                    if !match_pattern(&input.to_string(), i) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            return true;
        }
        if match_pattern(&letter.to_string(), &pattern[0]) {
            for (x, i) in pattern.iter().enumerate() {
                if let Some(input) = inputline.chars().nth(x + index) {
                    if !match_pattern(&input.to_string(), i) {
                        break;
                    } else if x == pattern.len() - 1 {
                        return true;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    return false;
}
>>>>>>> f514c10 (codecrafters submit [skip ci])
