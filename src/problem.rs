pub trait AoCProblem {
    fn parse_line(&mut self, line: String);
    fn solve_part1(&self) -> i64;
    fn solve_part2(&self) -> i64;
}
