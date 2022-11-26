// advent of code website: https://adventofcode.com/2022
// github: https://github.com/mullen25312/aoc2022_rust

// daily puzzle day: d01
fn main() {
    println!("########### d01 ###########");

    let file = String::from("./demo.txt");
    // let file = String::from("./input.txt");

    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    // let data: Vec<i32> = contents.lines().map(str::parse).map(Result::unwrap).collect();
}
