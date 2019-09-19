use crate::helper;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Hash)]
pub enum LetterCount {
    None,
    Double,
    Triple,
    DoubleAndTriple
}

pub fn solve_problem_2(file_path: &str) -> (Option<i32>, Option<String>)  {
    let content = helper::get_file_content_as_str(file_path);
    if content.is_none() { return (None, None); }

    let content = content.unwrap();
    let checksum = get_checksum(&content);

    let id = get_id(&content);
    (checksum, id)
}

pub fn get_id(content: &str) -> Option<String> {
    for id in content.lines() {
        for next in content.lines() {
            if id == next { continue; }
            let mut different_chars = 0;
            let mut last_different_char = None;
            let chars = id.chars().zip(next.chars());
            for (ch1, ch2) in chars {
                if ch1 != ch2 {
                    different_chars += 1;
                    last_different_char = Some(ch1)
                }
            }

            if different_chars == 1 {
                // it is safe to just unwrap the last_different_char, because if the different_chars is
                // greater than 0, it is holding some value, and if it is equal to one
                // it means that it was only changed once.
                let chars = id.chars()
                    .filter(|chars| *chars != last_different_char.unwrap());
                return Some(String::from_iter(chars));
            }
        }
    }

    None
}

pub fn get_checksum(content: &str) -> Option<i32> {
    let mut doubles = 0;
    let mut triples = 0;
    for line in content.lines(){
        let result = analyze_word(line);
        match result {
            LetterCount::Double => doubles += 1,
            LetterCount::Triple => triples += 1,
            LetterCount::DoubleAndTriple => {
                doubles += 1;
                triples += 1;
            }
            _ => ()
        }
    }
    Some(doubles * triples)
}

fn analyze_word(word: &str) -> LetterCount {
    let mut map = HashMap::new();
    for chars in word.chars() {
        map.entry(chars)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    }

    let mut is_double = false;
    let mut is_triple = false;
    for value in map.values() {
        if *value == 2 { is_double = true; }
        if *value == 3 { is_triple = true; }
    }
    match (is_double, is_triple) {
        (true, true) => LetterCount::DoubleAndTriple,
        (false, true) => LetterCount::Triple,
        (true, false) => LetterCount::Double,
        (false, false) => LetterCount::None
    }
}
