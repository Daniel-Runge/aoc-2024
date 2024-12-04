use std::fs::File;
use std::io::{BufRead, BufReader};

const SIZE: usize = 140;
const PADDING: usize = 3;
const LENGTH: usize = SIZE + PADDING * 2;

fn parse(filename: &str) -> Result<[[char; LENGTH]; LENGTH], std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut content_matrix: [[char; LENGTH]; LENGTH] = [[char::default(); LENGTH]; LENGTH];
    for (row_index, line) in reader.lines().enumerate() {
        for (column_index, character) in line?.chars().enumerate() {
            content_matrix[row_index + PADDING][column_index + PADDING] = character;
        }
    }

    Ok(content_matrix)
}

fn check_forward_horizontal(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i][j] == 'X'
        && matrix[i][j + 1] == 'M'
        && matrix[i][j + 2] == 'A'
        && matrix[i][j + 3] == 'S'
    {
        return 1;
    }
    0
}

fn check_backward_horizontal(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i][j] == 'X'
        && matrix[i][j - 1] == 'M'
        && matrix[i][j - 2] == 'A'
        && matrix[i][j - 3] == 'S'
    {
        return 1;
    }
    0
}

fn check_upward_horizontal(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i][j] == 'X'
        && matrix[i + 1][j] == 'M'
        && matrix[i + 2][j] == 'A'
        && matrix[i + 3][j] == 'S'
    {
        return 1;
    }
    0
}

fn check_downward_horizontal(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i][j] == 'X'
        && matrix[i - 1][j] == 'M'
        && matrix[i - 2][j] == 'A'
        && matrix[i - 3][j] == 'S'
    {
        return 1;
    }
    0
}

fn check_forward_downward_diagonal(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i][j] == 'X'
        && matrix[i + 1][j + 1] == 'M'
        && matrix[i + 2][j + 2] == 'A'
        && matrix[i + 3][j + 3] == 'S'
    {
        return 1;
    }
    0
}

fn check_backward_downward_diagonal(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i][j] == 'X'
        && matrix[i - 1][j + 1] == 'M'
        && matrix[i - 2][j + 2] == 'A'
        && matrix[i - 3][j + 3] == 'S'
    {
        return 1;
    }
    0
}

fn check_forward_upward_diagonal(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i][j] == 'X'
        && matrix[i + 1][j - 1] == 'M'
        && matrix[i + 2][j - 2] == 'A'
        && matrix[i + 3][j - 3] == 'S'
    {
        return 1;
    }
    0
}

fn check_backward_upward_diagonal(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i][j] == 'X'
        && matrix[i - 1][j - 1] == 'M'
        && matrix[i - 2][j - 2] == 'A'
        && matrix[i - 3][j - 3] == 'S'
    {
        return 1;
    }
    0
}

pub fn day_4_puzzle_1(filename: &str) {
    let mut solution = 0;
    let input = parse(filename).unwrap();
    for i in PADDING..LENGTH {
        for j in PADDING..LENGTH {
            solution += check_forward_horizontal(i, j, input);
            solution += check_backward_horizontal(i, j, input);
            solution += check_upward_horizontal(i, j, input);
            solution += check_downward_horizontal(i, j, input);
            solution += check_forward_downward_diagonal(i, j, input);
            solution += check_backward_downward_diagonal(i, j, input);
            solution += check_forward_upward_diagonal(i, j, input);
            solution += check_backward_upward_diagonal(i, j, input);
        }
    }

    println!("Day 4 Puzzle 1 solution: {solution}");
}

fn check_mas_mas(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i - 1][j - 1] == 'M'
        && matrix[i + 1][j + 1] == 'S'
        && matrix[i + 1][j - 1] == 'M'
        && matrix[i - 1][j + 1] == 'S'
    {
        return 1;
    };
    0
}

fn check_mas_sam(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i - 1][j - 1] == 'M'
        && matrix[i + 1][j + 1] == 'S'
        && matrix[i + 1][j - 1] == 'S'
        && matrix[i - 1][j + 1] == 'M'
    {
        return 1;
    };
    0
}
fn check_sam_mas(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i - 1][j - 1] == 'S'
        && matrix[i + 1][j + 1] == 'M'
        && matrix[i + 1][j - 1] == 'M'
        && matrix[i - 1][j + 1] == 'S'
    {
        return 1;
    };
    0
}
fn check_sam_sam(i: usize, j: usize, matrix: [[char; LENGTH]; LENGTH]) -> i32 {
    if matrix[i - 1][j - 1] == 'S'
        && matrix[i + 1][j + 1] == 'M'
        && matrix[i + 1][j - 1] == 'S'
        && matrix[i - 1][j + 1] == 'M'
    {
        return 1;
    };
    0
}

pub fn day_4_puzzle_2(filename: &str) {
    let mut solution = 0;
    let input = parse(filename).unwrap();
    for i in PADDING..LENGTH {
        for j in PADDING..LENGTH {
            if input[i][j] == 'A' {
                solution += check_mas_mas(i, j, input);
                solution += check_mas_sam(i, j, input);
                solution += check_sam_mas(i, j, input);
                solution += check_sam_sam(i, j, input);
            }
        }
    }

    println!("Day 4 Puzzle 2 solution: {solution}");
}
