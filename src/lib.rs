use std::io;

pub struct Config {
    pub pattern: String,
    pub input_line: String,
}
enum WhatWematchin {
    Exact(String),
    EndOfLine(String),
    BeginningOfLine(String),
    Group(String),
    Symbol(String),
}
impl Config {
    pub fn new(input: &[String]) -> Result<Config, &'static str> {
        if input.len() < 3 {
            return Err("not enough arguments");
        }
        let pattern = input[2].clone();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line = input_line.trim_end_matches('\n').to_string();

        Ok(Config {
            pattern,
            input_line: input_line.to_string(),
        })
    }
    fn serialize_pattern(pattern: &str) -> Vec<WhatWematchin> {
        let pattern: Vec<&str> = pattern.split("").filter(|x| *x != "").collect();
        let mut groups = String::new();
        let mut temp = Vec::new();
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
                temp.push(i.to_string());
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

        if !temp.is_empty() {
            final_pat.push(temp.join(""));
        }
        dbg!(&final_pat);
        let mut pattern_enum = Vec::new();
        for pat in &final_pat {
            if pat.starts_with("[") {
                pattern_enum.push(WhatWematchin::Group(String::from(pat)));
            } else if pat.starts_with("\\") {
                pattern_enum.push(WhatWematchin::Symbol(String::from(pat)));
            } else if pat.starts_with("^") {
                pattern_enum.push(WhatWematchin::BeginningOfLine(String::from(pat)));
            } else if pat.ends_with("$") {
                pattern_enum.push(WhatWematchin::EndOfLine(String::from(pat)));
            } else if pat.starts_with("[") {
                pattern_enum.push(WhatWematchin::Group(String::from(pat)));
            } else {
                pattern_enum.push(WhatWematchin::Exact(String::from(pat)));
            }
        }
        return pattern_enum;
    }
}
pub fn grep(input: &str, pattern: &str) -> bool {
    let pattern = Config::serialize_pattern(pattern);
    for pat in pattern {
        let flag = match pat {
            WhatWematchin::Exact(pattern) => match_exact(input, &pattern),
            WhatWematchin::EndOfLine(pattern) => line_matches(input, &pattern, true),
            WhatWematchin::BeginningOfLine(pattern) => line_matches(input, &pattern, false),
            WhatWematchin::Group(pattern) => match_group(input, &pattern),
            WhatWematchin::Symbol(pattern) => match_symbol(input, &pattern),
        };
        if flag == false {
            return false;
        }
    }
    return true;
}
fn match_exact(input: &str, pattern: &str) -> bool {
    println!("matchin :{input} with :{pattern}");
    let mut iter = input.chars();
    for ch in pattern.chars() {
        match iter.find(|&x| x == ch) {
            Some(_) => continue,
            None => return false,
        }
    }
    true
}
fn line_matches(input: &str, pattern: &str, end: bool) -> bool {
    let mut input: Vec<&str> = input.split("").collect();
    let mut pattern: Vec<&str> = pattern.split("").collect();
    if end {
        input.reverse();
        pattern.reverse();
    }
    let input = input.join("");
    let pattern = pattern.join("");

    println!("matchin :{input} with :{pattern}");
    for (i, ch) in pattern[1..].chars().enumerate() {
        println!("LOOP ch :{ch}");
        if let Some(x) = input.chars().nth(i) {
            println!("LOOP x:{x}");
            if x != ch {
                return false;
            }
        } else {
            println!("LOOP x:NOT FOUND");
            return false;
        }
    }
    true
}
fn match_group(input: &str, pattern: &str) -> bool {
    println!("matching {input} with group {pattern}");
    if pattern.starts_with("[^") {
        return pattern[1..pattern.len() - 1]
            .chars()
            .all(|x| !input.contains(x));
    }
    let pattern = &pattern[1..pattern.len() - 1];
    for c in pattern.chars() {
        if input.contains(c) {
            return true;
        }
    }
    return false;
}
fn match_symbol(input: &str, pattern: &str) -> bool {
    match pattern {
        r#"\w"# => input.contains(|c: char| c.is_alphanumeric()),
        r#"\d"# => input.contains(|c: char| c.is_digit(10)),
        _ => false,
    }
}
//--------------------------------------------------------------//
//TESTS
//______________________________________________________________//

#[cfg(test)]
pub mod exact {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(grep("wd", "d"), true);
    }

    #[test]
    fn case2() {
        assert!(grep("ass", "ass"));
    }
}
#[cfg(test)]

pub mod symbols {
    use super::*;
    #[test]
    fn case3() {
        assert!(grep("2", "\\d"));
    }
    #[test]
    fn case4() {
        assert!(grep("012", "\\d\\d\\d"));
    }
}
#[cfg(test)]
pub mod groups {
    use super::*;

    #[test]
    fn case5() {
        assert_ne!(grep("w29d", "[sa]"), true);
    }
    #[test]
    fn case6() {
        assert!(grep("oopspp", "[so]"));
    }
    #[test]
    fn case7() {
        assert!(grep("019248apapopopiw23", "[^nmbv]"));
    }
    #[test]
    fn case8() {
        assert!(grep("qwe", "[sw]"));
    }
}
#[cfg(test)]
pub mod combinations {

    use super::*;
    #[test]
    fn case9() {
        assert!(grep("d2d apple", "\\w\\d\\w apple"));
    }
    #[test]
    fn case10() {
        assert!(grep("22w a", "\\d\\dw [sa]"));
    }
    #[test]
    fn case11() {
        assert_ne!(grep("opac", "[^c]"), true);
    }
    #[test]
    fn case15() {
        assert_ne!(grep("dsx", "d[pw]x"), true);
    }
    #[test]
    fn case16() {
        assert_ne!(grep("12 ds 21", "12 ds [^2]1"), true);
    }
    #[test]
    fn case17() {
        assert_ne!(grep("22w ", "\\d\\dw [^sa]"), true);
    }
}
#[cfg(test)]
pub mod beginning_of_line {
    use super::*;
    #[test]
    fn case12() {
        assert_eq!(grep("opac", "^opa"), true);
    }
    #[test]
    fn case13() {
        assert_eq!(grep("opac", "^o"), true);
    }
    #[test]
    fn case14() {
        assert_eq!(grep("a", "^a"), true);
    }
    #[test]
    fn case18() {
        assert_ne!(grep("da", "^das"), true);
    }
    #[test]
    fn case19() {
        assert_ne!(grep("ad", "^d"), true);
    }
    #[test]
    fn case20() {
        assert_ne!(grep("1p", "^1 "), true);
    }
    #[test]
    fn case21() {
        assert_ne!(grep("daas", "^aas"), true);
    }
    #[test]
    fn case22() {
        assert_ne!(grep("slog", "^log"), true);
    }
}

#[cfg(test)]
pub mod end_of_line {
    use super::*;

    #[test]
    fn case23() {
        assert_ne!(grep("man ", "man$"), true);
    }
    #[test]
    fn case24() {
        assert!(grep("o", "o$"));
    }
    #[test]
    fn case25() {
        assert!(grep("mad man", "man$"));
    }
    #[test]
    fn case26() {
        assert!(grep("qwe  ", "  $"));
    }
}
