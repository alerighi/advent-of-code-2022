use crate::problem::AoCProblem;

#[derive(Debug, Copy, Clone)]
struct Move {
    elements: usize,
    from: usize,
    to: usize,
}

#[derive(Default, Debug)]
pub struct AoCDay5 {
    stacks: [Vec<char>; 10],
    moves: Vec<Move>
}

impl AoCProblem for AoCDay5 {
    fn parse_line(&mut self, line: String) {
        if line.starts_with("move") {
            let parts: Vec<&str> = line.split(" ").collect();
            self.moves.push(Move {
                elements: parts[1].parse().unwrap(),
                from: parts[3].parse::<usize>().unwrap() - 1,
                to: parts[5].parse::<usize>().unwrap() - 1,
            })
        } else {
            for (i, c) in line.chars().enumerate() {   
                if i % 4 == 1 && c.is_alphabetic() {
                    self.stacks[i / 4].insert(0, c);
                }
            }
        }
    }

    fn solve_part1(&self) -> String {
        let mut stacks = self.stacks.clone(); 
        for m in &self.moves {
            for _ in 0..m.elements {
                let el = stacks[m.from].pop().unwrap();
                stacks[m.to].push(el);
            }
        }

        let mut result = String::new();
        for s in stacks {
            if s.len() > 0 {
                result.push(s[s.len() - 1]);
            }
        }
        result
    }

    fn solve_part2(&self) -> String {
        let mut stacks = self.stacks.clone(); 
        for m in &self.moves {
            let start = stacks[m.from].len() - m.elements;
            let end = stacks[m.from].len();
            let mut el: Vec<char> = stacks[m.from].drain(start..end).collect();
            
            stacks[m.to].append(&mut el);
        }

        let mut result = String::new();
        for s in stacks {
            if s.len() > 0 {
                result.push(s[s.len() - 1]);
            }
        }
        result
    }
}
