use crate::problem::AoCProblem;

const START_POSITION: (usize, usize) = (500, 0);
const GRID_MAX_SIZE: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Sand
}

type Grid = [[Cell; GRID_MAX_SIZE]; GRID_MAX_SIZE];

#[derive(Debug)]
pub struct AoCDay14 {
    grid: Grid,
    max_y: usize,
}

impl Default for AoCDay14 {
    fn default() -> Self {
        Self { 
            grid: [[Cell::Empty; GRID_MAX_SIZE]; GRID_MAX_SIZE],
            max_y: 0,
        }
    }
}


fn drop_sand(grid: &mut Grid) -> bool {
    let (mut x, mut y) = START_POSITION;
    let mut can_move = true;
    while can_move {
        let candidates = [
            (x, y + 1),
            (x - 1, y + 1),
            (x + 1, y + 1),
        ];
        can_move = false;
        for (cx, cy) in candidates {
            // reached bottom of grid!
            if cy >= GRID_MAX_SIZE {
                return false;
            }

            if cx < GRID_MAX_SIZE && grid[cy][cx] == Cell::Empty {
                (x, y) = (cx, cy);
                can_move = true;
                break;
            }
        }
    }
    // can no more move to the bottom. Place sand piece
    grid[y][x] = Cell::Sand;
    true
}

impl AoCProblem for AoCDay14 {
    fn parse_line(&mut self, line: String) {
        let mut points: Vec<(usize, usize)> = Vec::new();
        for part in line.split("->") {
            let coord: Vec<usize> = part.trim().split(",").map(|v| v.parse().unwrap()).collect();
            points.push((coord[0], coord[1]));
            self.max_y = self.max_y.max(coord[1]);
        }
        for i in 1..points.len() {
            let mut a = points[i - 1];
            let mut b = points[i];
            if a.0 > b.0 || a.1 > b.1 {
                (a, b) = (b, a)
            }
            loop {
                self.grid[a.1][a.0] = Cell::Wall;
                if a == b {
                    break;
                }
                if a.0 == b.0 {
                    a.1 += 1;
                } else {
                    a.0 += 1;
                }
            }
        }
    }

    fn solve_part1(&self) -> String {
        let mut grid = self.grid.clone();
        let mut result = 0;
        while drop_sand(&mut grid) {
            result += 1;
        }
        result.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut grid = self.grid.clone();
        for i in 0..GRID_MAX_SIZE {
            grid[self.max_y + 2][i] = Cell::Wall;
        }
        let mut result = 0;
        while grid[START_POSITION.1][START_POSITION.0] != Cell::Sand && drop_sand(&mut grid) {
            result += 1;
        }
        result.to_string()
    }
}
