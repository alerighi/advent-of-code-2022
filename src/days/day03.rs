use crate::problem::AoCProblem;
use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct AoCDay3 {
    lines: Vec<String>,
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => 1 + c as u32 - 'a' as u32,
        'A'..='Z' => 27 + c as u32 - 'A' as u32,
        _ => unreachable!()
    }
}

fn common_chars(a: &str, b: &str) -> HashSet<char> {
    a.chars().filter(|c| b.contains(*c)).collect()
}

impl AoCProblem for AoCDay3 {
    fn parse_line(&mut self, line: String) {
        self.lines.push(line)
    }

    fn solve_part1(&self) -> i64 {
        let mut result = 0;
        for line in &self.lines {
            let (sack1, sack2) = line.split_at(line.len() / 2);
            assert_eq!(sack1.len(), sack2.len());

            result += common_chars(sack1, sack2).into_iter().map(|c| priority(c) as i64).sum::<i64>();
        }
        result
    }

    fn solve_part2(&self) -> i64 {
        assert_eq!(self.lines.len() % 3, 0);
        let mut result = 0;
        for i in 0..(self.lines.len() / 3) {
            for c in common_chars(self.lines[i * 3].as_str(), self.lines[i * 3 + 1].as_str()) {
                if self.lines[i * 3 + 2].contains(c) {
                    result += priority(c) as i64;
                }
            }
        }
        result
    }
}
