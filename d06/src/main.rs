// advent of code website: https://adventofcode.com/2022
// github: https://github.com/mullen25312/aoc2022_rust

use itertools::Itertools;

fn check4duplicates(window: &[char]) -> bool {
    for ((idx1, char1), (idx2, char2)) in window.iter().enumerate().tuple_combinations() {
        if char1 == char2 && idx1 != idx2 {
            return true;
        }
    }
    return false;
}

fn find_marker(data: &String, size: usize) -> usize {
    data.chars().collect::<Vec<char>>().windows(size).map(|x| !check4duplicates(x)).position(|x| x).unwrap() + size
}

// daily puzzle day: d06
fn main() {
    println!("########### d06 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    // parse input
    let data = std::fs::read_to_string(file).expect("Should have been able to read the file");

    // part one
    println!("Result of part one: {:?}", find_marker(&data, 4));

    // part two
    println!("Result of part one: {}", find_marker(&data, 14));
}
