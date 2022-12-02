// advent of code website: https://adventofcode.com/2022
// github: https://github.com/mullen25312/aoc2022_rust

use std::collections::HashMap;

// daily puzzle day: d02
fn main() {
    // 1: Rock (A, X), 2: Paper (B, Y), 3: Scissor (C, Z)
    let hand2number: HashMap<&str, i32> = HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    let win_eval: HashMap<[i32; 2], i32> =
        HashMap::from([([1, 1], 3), ([1, 2], 6), ([1, 3], 0), ([2, 1], 0), ([2, 2], 3), ([2, 3], 6), ([3, 1], 6), ([3, 2], 0), ([3, 3], 3)]);

    // 1: lose (0 points), 2: draw (3 points), 3: win (6 points)
    let what2play: HashMap<[i32; 2], i32> =
        HashMap::from([([1, 1], 3), ([1, 2], 1), ([1, 3], 2), ([2, 1], 1), ([2, 2], 2), ([2, 3], 3), ([3, 1], 2), ([3, 2], 3), ([3, 3], 1)]);
    let points: HashMap<i32, i32> = HashMap::from([(1, 0), (2, 3), (3, 6)]);

    println!("########### d02 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    // parse input
    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut games: Vec<[i32; 2]> = Vec::new();
    for line in lines {
        let tmp: Vec<&str> = line.split(" ").collect();
        games.push([*hand2number.get(tmp[0]).unwrap_or(&1), *hand2number.get(tmp[1]).unwrap_or(&1)]);
    }

    // part one
    let result1: i32 = (games.iter().map(|game| win_eval[game] + game[1])).sum::<i32>();
    println!("Result of part one: {}", result1);

    // part two
    let result2: i32 = (games.iter().map(|game| what2play[game] + points[&game[1]])).sum::<i32>();
    println!("Result of part one: {}", result2);
}
