use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

fn parse(filename: &str) -> Result<(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>), std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut updates = Vec::<Vec<i32>>::new();
    for line in reader.lines() {
        let line = line?;
        if let [first, second, ..] = &line.split("|").map(String::from).collect::<Vec<String>>()[..]
        {
            let first = first.parse::<i32>().unwrap();
            let second = second.parse::<i32>().unwrap();

            // rules.entry(first).or_insert_with(HashSet::new).insert(second);

            match rules.entry(second) {
                Entry::Vacant(entry) => {
                    entry.insert(HashSet::from([first]));
                }
                Entry::Occupied(mut entry) => {
                    entry.get_mut().insert(first);
                }
            }
        } else if let Ok(update) = &line
            .split(",")
            .map(|string| string.parse::<i32>())
            .collect::<Result<Vec<i32>, ParseIntError>>()
        {
            updates.push(update.clone())
        }
    }

    Ok((rules, updates))
}

pub fn day_5_puzzle_1(filename: &str) {
    let (rules, updates) = parse(filename).unwrap();
    let result = updates
        .into_iter()
        .filter(|page| {
            page.is_sorted_by(|first, second| !rules.get(first).unwrap().contains(second))
        })
        .map(|page| page[page.len() / 2])
        .sum::<i32>();

    println!("Day 5 Puzzle 1 solution: {result}");
}

pub fn day_5_puzzle_2(filename: &str) {
    let (rules, updates) = parse(filename).unwrap();
    let result = updates
        .into_iter()
        .filter(|page| {
            !page.is_sorted_by(|first, second| {
                // println!("{:?} -> {:?}", first, second);
                !rules.get(first).unwrap().contains(second)
            })
        })
        .map(|mut incorrect_page| {
            incorrect_page.sort_by(|first, second| {
                if let Some(before) = rules.get(first) {
                    if before.contains(second) {
                        return Ordering::Less;
                    }
                }
                if let Some(after) = rules.get(second) {
                    if after.contains(first) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });
            incorrect_page
        })
        .map(|page| page[page.len() / 2])
        .sum::<i32>();

    println!("Day 5 Puzzle 2 solution: {result}");
}
