use crate::problem::AoCProblem;
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn distance(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Sensor {
    position: Point,
    beacon: Point,
    d: i32,
}

#[derive(Debug, Default)]
pub struct AoCDay15 {
    sensors: Vec<Sensor>,
    y_range: (i32, i32),
    x_range: (i32, i32),
}

fn perimeter(center: Point, distance: i32) -> HashSet<Point> {
    let mut result: HashSet<Point> = HashSet::new();
    for d in 0..=distance {
        result.insert(Point {
            x: center.x + d,
            y: center.y + (distance - d),
        });
        result.insert(Point {
            x: center.x - d,
            y: center.y + (distance - d),
        });
        result.insert(Point {
            x: center.x + d,
            y: center.y - (distance - d),
        });
        result.insert(Point {
            x: center.x - d,
            y: center.y - (distance - d),
        });
    }
    result
}

impl AoCProblem for AoCDay15 {
    fn parse_line(&mut self, line: String) {
        let parsed = sscanf::sscanf!(line, "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}").unwrap();
        let beacon = Point {
            x: parsed.2,
            y: parsed.3,
        };
        let position = Point {
            x: parsed.0,
            y: parsed.1,
        };
        let d = distance(position, beacon);
        self.sensors.push(Sensor {
            position,
            beacon,
            d,
        });
        self.y_range = (self.y_range.0.min(beacon.y.min(position.y) - d), self.y_range.1.max(beacon.y.max(position.y) + d));
        self.x_range = (self.x_range.0.min(beacon.x.min(position.x) - d), self.x_range.1.max(beacon.x.max(position.x) + d));
    }

    fn solve_part1(&self) -> String {
        let mut result: u64 = 0;
        let y = if self.sensors.len() < 15 {
            10
        } else {
            2000000
        };

        for x in self.x_range.0..=self.x_range.1 {
            let p = Point { x, y };
            let mut covered = false;
            for s in &self.sensors {
                if p == s.beacon {
                    // I'm a beacon!
                    covered = false;
                    break;
                }
                if !covered && distance(p, s.position) <= s.d {
                    covered = true;
                }
            }
            if covered {
                result += 1; 
            }
        }
        result.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut ban: HashSet<Point> = HashSet::new();
        let max = if self.sensors.len() < 15 {
            20
        } else {
            4000000
        };

        for sensor in &self.sensors {
            for candidate in perimeter(sensor.position, sensor.d + 1) {
                if candidate.x >= 0 && candidate.y >= 0 && candidate.y <= max && candidate.x <= max && !ban.contains(&candidate) {
                    let mut found = true;
                    for s in &self.sensors {
                        if distance(candidate, s.position) <= s.d {
                            ban.insert(candidate);
                            found = false;
                            break;
                        }
                    }
                    if found {
                        return ((candidate.x as u64) * 4000000 + (candidate.y as u64)).to_string()
                    }
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::{perimeter, Point};

    #[test]
    fn test_perimter() {
        let result = perimeter(Point { x: 0, y: 0 }, 1);
        assert_eq!(result.len(), 4);
        assert!(result.contains(&Point { x: 1, y: 0 }));
        assert!(result.contains(&Point { x: -1, y: 0 }));
        assert!(result.contains(&Point { x: 0, y: 1 }));
        assert!(result.contains(&Point { x: 0, y: -1 }));
    }
}