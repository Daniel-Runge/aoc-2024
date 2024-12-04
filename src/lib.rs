mod day_one;
mod day_two;
mod day_three;
mod day_four;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
pub struct AocDay {
    pub day: usize,
    puzzle_1_solution: fn(&str),
    puzzle_2_solution: fn(&str),
}

impl AocDay {
    pub fn all_days() -> [AocDay; 4] {
        [
            AocDay {
                day: 1,
                puzzle_1_solution: day_one::day_1_puzzle_1,
                puzzle_2_solution: day_one::day_1_puzzle_2,
            },
            AocDay {
                day: 2,
                puzzle_1_solution: day_two::day_2_puzzle_1,
                puzzle_2_solution: day_two::day_2_puzzle_2,
            },
            AocDay {
                day: 3,
                puzzle_1_solution: day_three::day_3_puzzle_1,
                puzzle_2_solution: day_three::day_3_puzzle_2,
            },
            AocDay {
                day: 4,
                puzzle_1_solution: day_four::day_4_puzzle_1,
                puzzle_2_solution: day_four::day_4_puzzle_2,
            }
        ]
    }

    pub fn solve(&self, filename: &str) {
        let puzzle_one_instant = Instant::now();
        (self.puzzle_1_solution)(filename);
        let elapsed_one = puzzle_one_instant.elapsed();
        println!("Day {:?} Puzzle 1 solved in: {:.2?}", self.day, elapsed_one);

        let puzzle_two_instant = Instant::now();
        (self.puzzle_2_solution)(filename);
        let elapsed_two = puzzle_two_instant.elapsed();
        println!("Day {:?} Puzzle 2 solved in: {:.2?}", self.day, elapsed_two);
    }
}
