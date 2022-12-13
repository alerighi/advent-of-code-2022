use crate::problem::AoCProblem;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AoCDay12 {
    elevation: Vec<Vec<u32>>,
    start: (i32, i32),
    end: (i32, i32),
}

impl AoCDay12 {
    fn neighbors(&self, v: (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbours = Vec::new();

        let max_x = self.elevation[0].len() as i32 - 1;
        let max_y = self.elevation.len() as i32 - 1;
        let mut elevation = self.elevation[v.1 as usize][v.0 as usize];
        if elevation == 0 {
            elevation = 1;
        }

        let (x, y) = v;
        for u in [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1)
        ] {
            let (x, y) = u;
            if x >= 0 && y >= 0 && x <= max_x && y <= max_y
                && self.elevation[y as usize][x as usize] >= elevation - 1 {
                neighbours.push(u);
            }
        }

        neighbours
    }

    fn shortest_path(&self) -> HashMap<(i32, i32), u32> {
        let mut q: BinaryHeap<(i32, (i32, i32))> = BinaryHeap::new();
        let mut d: HashMap<(i32, i32), u32> = HashMap::new();

        q.push((0, self.end));
        d.insert(self.end,  0);

        while let Some((cost_, v)) = q.pop() {
            let cost = -cost_ as u32;
            if cost > *d.get(&v).unwrap_or(&u32::MAX) {
                continue
            }

            for u in self.neighbors(v) {
                let c = cost + 1;
                if c < *d.get(&u).unwrap_or(&u32::MAX) {
                    d.insert(u, c);
                    q.push((-(c as i32), u));
                }
            }
        }
        d
    }
}

impl AoCProblem for AoCDay12 {
    fn parse_line(&mut self, line: String) {
        let mut row = Vec::new();
        for (i, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    self.start = (i as i32, self.elevation.len() as i32);
                    row.push(0);
                },
                'E' => {
                    self.end = (i as i32, self.elevation.len() as i32);
                    row.push('z' as u32 - 'a' as u32);
                },
                'a'..='z' => {
                    row.push(ch as u32 - 'a' as u32);
                },
                _ => panic!("invalid input"),
            }
        }
        self.elevation.push(row);
    }

    fn solve_part1(&self) -> String {
        self.shortest_path().get(&self.start).unwrap().to_string()
    }

    fn solve_part2(&self) -> String {
        let d = self.shortest_path();
        let mut response = u32::MAX;
        for (y, e) in self.elevation.iter().enumerate() {
            for (x, e) in e.iter().enumerate() {
                if e == &0 {
                    response = response.min(*d.get(&(x as i32, y as i32)).unwrap_or(&u32::MAX));
                }
            }
        }
        response.to_string()
    }
}
