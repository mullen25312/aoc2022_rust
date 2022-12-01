// advent of code website: https://adventofcode.com/2022
// github: https://github.com/mullen25312/aoc2022_rust

// daily puzzle day: d01
fn main() {
    println!("########### d01 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    // parse input
    let mut elves: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for line in lines {
        if line != "" {
            tmp.push(line.parse().unwrap_or(0))
        } else {
            elves.push(tmp.clone());
            tmp.clear();
        }
    }
    elves.push(tmp.clone());

    elves.sort_by_key(|elf| -elf.iter().sum::<i32>());

    // part one
    let result1: i32 = elves[0].iter().sum::<i32>();
    println!("Result of part one: {}", result1);

    // part two
    let result2: i32 = elves[0].iter().sum::<i32>() + elves[1].iter().sum::<i32>() + elves[2].iter().sum::<i32>();
    println!("Result of part one: {}", result2);
}
