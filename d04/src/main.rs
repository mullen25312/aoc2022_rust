// advent of code website: https://adventofcode.com/2022
// github: https://github.com/mullen25312/aoc2022_rust

fn fully_overlaps(range1: &Vec<i32>, range2: &Vec<i32>) -> bool {
    range2[0] >= range1[0] && range2[1] <= range1[1] || range2[0] <= range1[0] && range2[1] >= range1[1]
}

fn overlaps(range1: &Vec<i32>, range2: &Vec<i32>) -> bool {
    range2[0] <= range1[1] && range2[1] <= range1[1] && range2[1] >= range1[0] || range1[0] <= range2[1] && range1[1] <= range2[1] && range1[1] >= range2[0]
}

// daily puzzle day: d04
fn main() {
    println!("########### d04 ###########");

    let file = String::from("./demo.txt");
    // let file = String::from("./input.txt");

    // parse input
    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut ranges: Vec<Vec<Vec<i32>>> = Vec::new();
    for line in &lines {
        ranges.push(line.split(",").map(|x| x.split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect());
    }

    // part one
    println!("Result of part one: {:?}", ranges.iter().map(|x| fully_overlaps(&x[0], &x[1])).filter(|x| *x).count());

    // part two
    println!("Result of part one: {}", ranges.iter().map(|x| overlaps(&x[0], &x[1])).filter(|x| *x).count());
}
