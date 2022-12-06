use crate::problem::AoCProblem;

type Pair = (u32, u32);

#[derive(Default, Debug)]
pub struct AoCDay4 {
    input: Vec<(Pair, Pair)>
}

fn full_overlap(i1: &Pair, i2: &Pair) -> bool {
    // ensure larger interval is i2
    if i1.1 - i1.0 < i2.1 - i2.0 {
        full_overlap(i2, i1)
    } else {
        // i1   0xxxxxxxxx1
        // i2     0xxxx1
        i1.0 <= i2.0 && i1.1 >= i2.1
    }
}

fn overlap(i1: &Pair, i2: &Pair) -> bool {
    if i1.0 > i2.0 {
        overlap(i2, i1)
    } else {
        // i1 0xxxxx1
        // i2   0xxxxx1
        i2.0 <= i1.1
    }
}

impl AoCProblem for AoCDay4 {
    fn parse_line(&mut self, line: String) {
        // ex. line fmt: 71-89,66-70
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts.len(), 2);

        fn parse_pair(p: &str) -> Pair {
            let parts: Vec<&str> = p.split('-').collect();
            assert_eq!(parts.len(), 2);
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        }

        self.input.push((parse_pair(parts[0]), parse_pair(parts[1])));
    }

    fn solve_part1(&self) -> String {
        self.input.iter().filter(|(i1, i2)| full_overlap(i1, i2)).count().to_string()
    }

    fn solve_part2(&self) -> String {
        self.input.iter().filter(|(i1, i2)| overlap(i1, i2)).count().to_string()
    }
}
