mod day_one;

#[derive(Debug, Copy, Clone)]
pub struct AocDay {
    pub day: usize,
    puzzle_1_solution: fn(&str),
    puzzle_2_solution: fn(&str),
}

impl AocDay {
    pub fn all_days() -> [AocDay; 1] {
        [AocDay {
            day: 1,
            puzzle_1_solution: day_one::day_1_puzzle_1,
            puzzle_2_solution: day_one::day_1_puzzle_2,
        }]
    }

    pub fn solve(&self, filename: &str) {
        (self.puzzle_1_solution)(filename);
        (self.puzzle_2_solution)(filename);
    }
}
