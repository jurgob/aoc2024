// https://adventofcode.com/2024/day/1

fn main() {
    println!("Result pt1 example: {}", part1(include_str!("../data/pt1_example.txt")));
    println!("Result pt1: {}", part1    (include_str!("../data/pt1_input.txt")));
    println!("Result pt2 example: {}", part2(include_str!("../data/pt2_example.txt")));
    println!("Result pt2: {}", part2(include_str!("../data/pt2_input.txt")));
}

fn part1(input: &str) -> i32 {
    let mut l_values:Vec<i32> = vec![];
    let mut r_values:Vec<i32> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            l_values.push(parts[0].parse::<i32>().unwrap());
            r_values.push(parts[1].parse::<i32>().unwrap());
        }
    }
    l_values.sort();
    r_values.sort();
    let mut distances: Vec<i32>= vec![];
    for i in 0..l_values.len() {
        let distance = if r_values[i] > l_values[i] {r_values[i] - l_values[i]} else {l_values[i] - r_values[i]};
        distances.push(distance);
    }
    let sum: i32 = distances.iter().sum();
    sum
}

fn part2(input: &str) -> i32 {
    let mut l_values:Vec<i32> = vec![];
    let mut r_values:Vec<i32> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            l_values.push(parts[0].parse::<i32>().unwrap());
            r_values.push(parts[1].parse::<i32>().unwrap());
        }
    }
    let similarity_scores = l_values.iter().map(|&l| r_values.iter().filter(|&&r| r == l).count() as i32);
    let result = l_values.iter().zip(similarity_scores).map(|(&l, s)| l * s).sum();
    result
}