use crate::problem::AoCProblem;

#[derive(Debug, Default)]
pub struct AoCDay6 {
    input: String
}

impl AoCProblem for AoCDay6 {
    fn parse_line(&mut self, line: String) {
        self.input = line;
    }

    fn solve_part1(&self) -> String {
        find_fist_marker(self.input.as_str(), 4).unwrap().to_string()
    }

    fn solve_part2(&self) -> String {
        find_fist_marker(self.input.as_str(), 14).unwrap().to_string()
    }
}

fn find_fist_marker(line: &str, min_chars: usize) -> Option<usize> {
    let mut queue: Vec<char> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        assert!(queue.len() < min_chars);
        queue.push(c);
    
        let mut elements: Vec<char> = queue.clone();
        elements.sort();
        elements.dedup();
        if elements.len() == min_chars {
            return Some(i + 1);
        }

        if queue.len() >= min_chars {
            queue.remove(0);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::find_fist_marker;

    #[test]
    fn test_part_1() {
        assert_eq!(find_fist_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), Some(7));
        assert_eq!(find_fist_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
        assert_eq!(find_fist_marker("nppdvjthqldpwncqszvftbrmjlhg",4 ), Some(6));
        assert_eq!(find_fist_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), Some(10));
        assert_eq!(find_fist_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(find_fist_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
        assert_eq!(find_fist_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
        assert_eq!(find_fist_marker("nppdvjthqldpwncqszvftbrmjlhg", 14 ), Some(23));
        assert_eq!(find_fist_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), Some(29));
        assert_eq!(find_fist_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), Some(26));
    }
}
