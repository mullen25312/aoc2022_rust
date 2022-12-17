// advent of code website: https://adventofcode.com/2022
// github: https://github.com/mullen25312/aoc2022_rust

use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn neighbours(&self, area: &Vec<Vec<i32>>) -> Vec<(Pos, i32)> {
        let height_of_area: i32 = area.len() as i32;
        let width_of_area: i32 = area[0].len() as i32;

        let &Pos(x, y) = self;

        let mut neighbors: Vec<Pos> = Vec::new();
        if (x - 1) >= 0 {
            if area[y as usize][x as usize] + 1 >= area[y as usize][(x - 1) as usize] {
                neighbors.push(Pos(x - 1, y));
            }
        }
        if (x + 1) < width_of_area {
            if area[y as usize][x as usize] + 1 >= area[y as usize][(x + 1) as usize] {
                neighbors.push(Pos(x + 1, y));
            }
        }
        if (y - 1) >= 0 {
            if area[y as usize][x as usize] + 1 >= area[(y - 1) as usize][x as usize] {
                neighbors.push(Pos(x, y - 1));
            }
        }
        if (y + 1) < height_of_area {
            if area[y as usize][x as usize] + 1 >= area[(y + 1) as usize][x as usize] {
                neighbors.push(Pos(x, y + 1));
            }
        }

        neighbors.into_iter().map(|p| (p, 1)).collect()
    }
}

// daily puzzle day: d12
fn main() {
    println!("########### d12 ###########");

    let file = String::from("./demo.txt");
    // let file = String::from("./input.txt");

    // parse input
    let contents = std::fs::read_to_string(file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let height_of_area: usize = lines.len();
    let width_of_area: usize = lines[0].len();
    let mut area: Vec<Vec<i32>> = vec![vec![0; width_of_area]; height_of_area];
    let mut start: Pos = Pos(0, 0);
    let mut starts: Vec<Pos> = Vec::new();
    let mut goal: Pos = Pos(0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == 'S' {
                area[y][x] = 'a' as i32 - 'a' as i32;
                start = Pos(x as i32, y as i32);
                starts.push(Pos(x as i32, y as i32));
            } else if character == 'E' {
                area[y][x] = 'z' as i32 - 'a' as i32;
                goal = Pos(x as i32, y as i32);
            } else if character == 'a' {
                area[y][x] = 'a' as i32 - 'a' as i32;
                starts.push(Pos(x as i32, y as i32));
            } else {
                area[y][x] = character as i32 - 'a' as i32;
            }
        }
    }

    // part one
    let result1 = dijkstra(&start, |p| p.neighbours(&area), |p| *p == goal);
    println!("Result of part one: {:?}", result1.unwrap_or_default().1);

    // part two
    let mut result2: Vec<i32> = Vec::new();

    for start_pos in starts {
        let tmp = dijkstra(&start_pos, |p| p.neighbours(&area), |p| *p == goal);
        if tmp.is_some() {
            result2.push(tmp.unwrap_or_default().1);
        }
    }
    result2.sort();

    println!("Result of part one: {}", result2[0]);
}
