// advent of code website: https://adventofcode.com/2022
// github: https://github.com/mullen25312/aoc2022_rust

// return duplicates of given strings as string
fn find_duplicate(items1: &String, items2: &String) -> String {
    let mut result: String = String::new();
    for char1 in items1.chars() {
        if items2.contains(char1) && !result.contains(char1) {
            result.push(char1);
        }
    }
    result
}

// return priority as follows a..z:1..26, A..Z:27..52
fn item_priority(item: char) -> i32 {
    if item.is_ascii_uppercase() {
        return item as i32 - 'A' as i32 + 27;
    } else {
        return item as i32 - 'a' as i32 + 1;
    }
}

// daily puzzle day: d03
fn main() {
    println!("########### d03 ###########");

    // let file = String::from("./demo.txt");
    let file = String::from("./input.txt");

    // parse input
    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();

    // part one
    let mut duplicates1: Vec<String> = Vec::new();
    for line in &lines {
        let (string1, string2): (&str, &str) = line.split_at(line.len() / 2);
        duplicates1.push(find_duplicate(&string1.to_string(), &string2.to_string()));
    }
    println!("Result of part one: {}", duplicates1.iter().map(|item| item_priority(item.chars().next().unwrap())).sum::<i32>());

    // part two
    let mut duplicates2: Vec<String> = Vec::new();
    for chunk in lines.chunks(3) {
        duplicates2.push(find_duplicate(&find_duplicate(&chunk[0], &chunk[1]), &chunk[2]));
    }
    println!("Result of part one: {}", duplicates2.iter().map(|item| item_priority(item.chars().next().unwrap())).sum::<i32>());
}
