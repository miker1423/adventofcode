use crate::helper;
use std::collections::HashSet;

pub fn solve_problem_1(path: &str) -> (Option<i32>, i32) {
    let content = helper::get_file_content_as_str(path);
    if content.is_none() { return (None, 0); }

    let content = content.unwrap();
    let content = content.as_str();
    let result_a = Some(calculate_frequency(content));

    let result_b = find_duplicated(content);
    (result_a, result_b)
}

pub fn calculate_frequency(data: &str) -> i32 {
    data.lines()
        .map(|line| line.parse::<i32>().unwrap())
        .fold(0, |acc, value| {
            acc + value
    })

    // I first had a version using the code below, it works great but using the
    // iterator methods to traverse the list of strings looks more 'elegant'
    // but maybe it could be something new to someone coming from other languages.
    //let mut result = 0;
    //for line in data.lines() {
    //    let value = line.parse::<i32>();
    //    if value.is_err() { return None; }
    //    let value = value.unwrap();
    //    result += value;
    //}
    //
    //Some(result)
}

pub fn find_duplicated(data: &str) -> i32 {
    let mut frequencies = HashSet::new();
    let mut current_frequency = 0;
    let not_found = true;
    while not_found {
        for line in data.lines() {
            let line = line.parse::<i32>().unwrap();
            current_frequency += line;
            if frequencies.contains(&current_frequency) { return current_frequency; }
            else { frequencies.insert(current_frequency); }
        }
    }
    // This will never be reached, because even if the frequency change does not repeats,
    // it will iterate again over the same data, so it will eventually reach a combination of
    // numbers that when added, will match a previously calculated frequency change.
    return 0;
}