use crate::problem::AoCProblem;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid string")
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Move {
    direction: Direction,
    len: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct State {
    points: Vec<Point>,
    tail_points: HashSet<Point>
}

impl State {
    fn new(size: usize) -> Self {
        Self {
            points: vec![Point::default(); size],
            tail_points: HashSet::new(),
        }
    }

    fn apply_move(&mut self, m: Move) {
        for _ in 0..m.len {
            match m.direction {
                Direction::Up => self.points[0].y += 1,
                Direction::Down => self.points[0].y -= 1,
                Direction::Right => self.points[0].x += 1,
                Direction::Left => self.points[0].x -= 1,
            }

            for i in 0..(self.points.len() - 1) {
                let head = self.points[i];
                let mut tail = &mut self.points[i + 1];

                let delta_x = tail.x - head.x;
                let delta_y = tail.y - head.y;

                if delta_x.abs() + delta_y.abs() > 2 {
                    if delta_x.abs() == 1 {
                        tail.x -= delta_x;
                    }
                    if delta_y.abs() == 1 {
                        tail.y -= delta_y;
                    }
                }

                if delta_y > 1 {
                    tail.y -= 1;
                } else if delta_y < -1 {
                    tail.y += 1;
                }
                if delta_x > 1 {
                    tail.x -= 1;
                } else if delta_x < -1 {
                    tail.x += 1;
                }
            }
            self.tail_points.insert(self.points[self.points.len() - 1]);
        }
    }

    fn touched_points(&self) -> usize {
        self.tail_points.len()
    }

}

#[derive(Debug, Default)]
pub struct AoCDay9 {
    moves: Vec<Move>
}

impl AoCProblem for AoCDay9 {
    fn parse_line(&mut self, line: String) {
        let parts: Vec<&str> = line.split(' ').collect();
        self.moves.push(Move {
            direction: Direction::from(parts[0]),
            len: parts[1].parse().unwrap(),
        });
    }

    fn solve_part1(&self) -> String {
        let mut state = State::new(2);
        for m in &self.moves {
            state.apply_move(*m);
        }
        state.touched_points().to_string()
    }

    fn solve_part2(&self) -> String {
        let mut state = State::new(10);
        for m in &self.moves {
            state.apply_move(*m);
        }
        state.touched_points().to_string()
    }
}
