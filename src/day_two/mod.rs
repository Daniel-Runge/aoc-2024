use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Report {
    safety: bool,
    levels: Vec<i32>,
}

impl Report {
    pub fn new(levels: Vec<i32>) -> Report {
        Report {
            safety: false,
            levels,
        }
    }
}

fn parse(filename: &str) -> Result<Vec<Report>, std::io::Error> {
    let mut result = Vec::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let report = line
            .split_whitespace()
            .map(|element| element.parse::<i32>().unwrap())
            .collect();
        result.push(Report::new(report));
    }

    Ok(result)
}

pub fn day_2_puzzle_1(filename: &str) {
    let mut reports = parse(&filename).unwrap();
    basic_report_safety_set(&mut reports);
    let number_of_safe_reports = reports.iter().filter(|report| report.safety).count();

    println!("Day 2 Puzzle 1 solution: {}", number_of_safe_reports);
}

pub fn day_2_puzzle_2(filename: &str) {
    let mut reports = parse(&filename).unwrap();
    basic_report_safety_set(&mut reports);
    let mut unsafe_reports = reports
        .clone()
        .into_iter()
        .filter(|report| !report.safety)
        .collect::<Vec<Report>>();

    for report in &mut unsafe_reports {
        let mut dampened_report_iterations = problem_dampener_iterations(&report);
        basic_report_safety_set(&mut dampened_report_iterations);
        report.safety = dampened_report_iterations
            .iter()
            .filter(|&report| report.safety)
            .count()
            > 0;
    }

    let number_of_safe_reports = reports.iter().filter(|report| report.safety).count();
    let number_of_safe_dampened_reports =
        unsafe_reports.iter().filter(|report| report.safety).count();

    println!(
        "Day 2 Puzzle 2 solution: {}",
        number_of_safe_reports + number_of_safe_dampened_reports
    );
}

fn basic_report_safety_set(reports: &mut Vec<Report>) {
    for report in reports {
        report.safety = check_ascending_safe(&report);
        if !report.safety {
            report.safety = check_descending_safe(&report);
        }
    }
}

fn problem_dampener_iterations(report: &Report) -> Vec<Report> {
    let mut dampened_iterations = Vec::new();
    for index in 0..report.levels.len() {
        let mut iteration = report.levels.clone();
        iteration.remove(index);
        dampened_iterations.push(Report::new(iteration));
    }

    dampened_iterations
}

fn check_ascending_safe(report: &Report) -> bool {
    report
        .levels
        .iter()
        .as_slice()
        .windows(2)
        .all(|element| element[0] <= element[1] && check_difference_safe(element[0], element[1]))
}

fn check_descending_safe(report: &Report) -> bool {
    report
        .levels
        .iter()
        .as_slice()
        .windows(2)
        .all(|element| element[0] >= element[1] && check_difference_safe(element[0], element[1]))
}

fn check_difference_safe(x: i32, y: i32) -> bool {
    let difference = (x - y).abs();
    difference <= 3 && difference >= 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_value() {
        let test_sequence = [89, 92, 94, 96, 97, 99].to_vec();
        let report = Report::new(test_sequence);
        let check = check_ascending_safe(&report);

        assert_eq!(report.safety, false);
        assert_eq!(check, true);
    }

    #[test]
    fn test_check_difference() {
        let test_sequence = [89, 90, 93].to_vec();
        let result1 = check_difference_safe(test_sequence[0], test_sequence[1]);
        let result2 = check_difference_safe(test_sequence[1], test_sequence[2]);

        assert_eq!(result1, true);
        assert_eq!(result2, true);
    }
}
