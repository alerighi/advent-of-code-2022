use crate::problem::AoCProblem;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct AoCDay8 {
    height: Vec<Vec<usize>>
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl AoCDay8 {
    fn count_trees(&self, x: i32, y: i32, height: usize, dir: Direction) -> usize {
        if y < 0 || x < 0 || y >= self.height.len() as i32 || x >= self.height[0].len() as i32 {
            0
        } else if self.height[y as usize][x as usize] >= height {
            1
        } else {
            1 + match dir {
                Direction::Up => self.count_trees(x, y - 1, height, dir),
                Direction::Down => self.count_trees(x, y + 1, height, dir),
                Direction::Left => self.count_trees(x - 1, y, height, dir),
                Direction::Right => self.count_trees(x + 1, y, height, dir)
            }
        }
    }

    fn view_score(&self, x: i32, y: i32) -> usize {
        let height = self.height[y as usize][x as usize];
        let mut score = 1;
        score *= self.count_trees(x + 1, y, height, Direction::Right);
        score *= self.count_trees(x - 1, y, height, Direction::Left);
        score *= self.count_trees(x, y + 1, height, Direction::Down);
        score *= self.count_trees(x, y - 1, height, Direction::Up);
        score
    }
}

impl AoCProblem for AoCDay8 {
    fn parse_line(&mut self, line: String) {
        let mut v = Vec::new();
        for c in line.chars() {
            v.push(c as usize - '0' as usize)
        }
        self.height.push(v);
    }

    fn solve_part1(&self) -> String {
        let max_y = self.height.len();
        let max_x  = self.height[0].len();
        let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

        for y in 0..max_y {
            let mut height = -1;
            for x in 0..max_x {
                let current = self.height[y][x] as i32;
                // more high than tree near in this direction: visible
                if current > height {
                    visible_trees.insert((x, y));
                }
                height = height.max(current);
            }

            height = -1;
            for x in (0..max_x).rev() {
                let current = self.height[y][x] as i32;
                // more high than tree near in this direction: visible
                if current > height {
                    visible_trees.insert((x, y));
                }
                height = height.max(current);
            }
        }

        for x in 0..max_x {
            let mut height = -1;
            for y in 0..max_y {
                let current = self.height[y][x] as i32;
                // more high than tree near in this direction: visible
                if current > height {
                    visible_trees.insert((x, y));
                }
                height = height.max(current);
            }

            height = -1;
            for y in (0..max_y).rev() {
                let current = self.height[y][x] as i32;
                // more high than tree near in this direction: visible
                if current > height {
                    visible_trees.insert((x, y));
                }
                height = height.max(current);
            }
        }

        visible_trees.len().to_string()
    }

    fn solve_part2(&self) -> String {
        let mut result = 0;
        for y in 0..self.height.len() {
            for x in 0..self.height[0].len() {
                result = result.max(self.view_score(x as i32, y as i32));
            }
        }
        result.to_string()
    }
}
