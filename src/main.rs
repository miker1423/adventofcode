mod problem1;
mod problem2;
mod problem4;
mod helper;

use clap::{Arg, App};
use std::path::PathBuf;

fn main() {
    let matches = App::new("Advent of Code")
        .arg(Arg::with_name("problem")
            .short("p")
            .long("problem")
            .value_name("problem_number")
            .takes_value(true)
            .help("Provide a problem number [1, 2, 4] to see result, the test file is at the project root.")
        ).get_matches();

    let problem_number = matches.value_of("problem").unwrap_or("1");
    let mut test_path = get_tests_path();
    match problem_number.parse().unwrap() {
        1 => run_problem1(&mut test_path),
        2 => run_problem2(&mut test_path),
        4 => run_problem4(&mut test_path),
        _ => println!("That problem is not implemented")
    }
}

fn get_tests_path() -> PathBuf {
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push("tests");
    current_dir
}

fn run_problem1(test_path: &mut PathBuf) {
    test_path.push("problem_1.txt");
    let result = problem1::solve_problem_1(test_path.to_str().unwrap());
    match result {
        (Some(result_a), result_b)
            => println!("The result of a is {} and the result of b is {}", result_a, result_b),
        (None, _) => println!("We had a problem opening the file or reading it.")
    }
}

fn run_problem2(test_path: &mut PathBuf) {
    test_path.push("problem_2.txt");
    let result = problem2::solve_problem_2(test_path.to_str().unwrap());
    match result {
        (Some(checksum), Some(id))
            => println!("the checksum is {} and the id is {}", checksum, id),
        (Some(checksum), None)
            => println!("the checksum is {} but it failed to find a valid id", {checksum}),
        (None, Some(id)) => println!("the checksum failed but the id is {}", id),
        _ => println!("Something greater failed")
    }
}

fn run_problem4(test_path: &mut PathBuf) {
    test_path.push("problem_4.txt");
    let (s1, s2) = problem4::solve_problem_4(test_path.to_str().unwrap());
    match (s1, s2) {
        (-1, -1) => println!("Failed to find guards"),
        (x, -1) => println!("Strategy 1 yielded {} but strategy 2 failed", x),
        (-1, y) => println!("Strategy 1 failed but strategy 2 yielded {}", y),
        (x, y) => println!("Strategy 1 yielded {} and strategy 2 yielded {}", x, y)
    }
}
