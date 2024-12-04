use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn parse(filename: &str) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(&file);
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let temp_line = line?;
        let mut split = temp_line.split_whitespace();
        let x = split.next().unwrap().to_owned();
        let y = split.next().unwrap().to_owned();

        first_list.push(x.parse().unwrap());
        second_list.push(y.parse().unwrap());
    }

    Ok((first_list, second_list))
}

pub fn day_1_puzzle_1(filename: &str) {
    let mut sum = 0;
    let (mut first_list, mut second_list) = parse(filename).unwrap();
    first_list.sort_unstable();
    second_list.sort_unstable();

    for (first, second) in zip(&first_list, &second_list) {
        sum += (first - second).abs();
    }

    println!("Day 1 Puzzle 1 solution: {sum}");
}

pub fn day_1_puzzle_2(filename: &str) {
    let mut similarity_score = 0;
    let (first_list, second_list) = parse(filename).unwrap();
    let mut hashmap: HashMap<i32, i32> = HashMap::new(); // number:count
    for element in &second_list {
        hashmap.insert(
            *element,
            1 + if hashmap.contains_key(element) {
                hashmap[element]
            } else {
                0
            },
        );
    }

    for element in &first_list {
        if hashmap.contains_key(&element) {
            similarity_score += element * hashmap[element];
        }
    }

    println!("Day 1 Puzzle 2 solution: {similarity_score}")
}
