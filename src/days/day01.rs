use crate::problem::AoCProblem;

#[derive(Debug)]
pub struct AoCDay1 {
    groups: Vec<Vec<u32>>,
}

impl Default for AoCDay1 {
    fn default() -> Self {
        Self {
            groups: vec![Vec::new()],
        }
    }
}

impl AoCProblem for AoCDay1 {
    fn parse_line(&mut self, line: String) {
        match line.parse() {
            Ok(value) => {
                self.groups.last_mut().unwrap().push(value);
            }
            Err(_) => {
                self.groups.push(Vec::new());
            }
        }
    }

    fn solve_part1(&self) -> i64 {
        self.groups
            .iter()
            .map(|el| el.iter().fold(0u32, |acc, el| acc + el))
            .fold(0, |acc, val| if val > acc { val } else { acc }) as i64
    }

    fn solve_part2(&self) -> i64 {
        let mut values: Vec<u32> = self
            .groups
            .iter()
            .map(|el| el.iter().fold(0u32, |acc, el| acc + el))
            .collect();
        values.sort_by(|a, b| b.cmp(a));

        (values[0] + values[1] + values[2]) as i64
    }
}
