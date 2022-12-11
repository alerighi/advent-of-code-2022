use crate::problem::AoCProblem;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn execute(&self, value: u64) -> u64 {
        match self {
            Operation::Add(v) => value + v,
            Operation::Mul(v) => value * v,
            Operation::Square => value * value,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn eval(&self, value: u64) -> usize {
        if value % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
struct State<'a> {
    monkeys: &'a Vec<Monkey>,
    worry_levels: Vec<Vec<u64>>,
    processed_items: Vec<u64>,
    divide_by: u64,
    modulus: u64,
}

impl State<'_> {
    fn new<'a>(monkeys: &'a Vec<Monkey>, divide_by: u64) -> State<'a> {
        let mut result: State<'a> = State {
            monkeys,
            worry_levels: vec![Vec::new(); monkeys.len()],
            processed_items: vec![0; monkeys.len()],
            divide_by,
            modulus: 1,
        };
        for (i, monkey) in monkeys.iter().enumerate() {
            result.worry_levels[i].extend(monkey.starting_items.iter());
            result.modulus *= monkey.test.divisible_by;
        }
        result
    }

    fn play_monkey(&mut self, i: usize) {
        let monkey = &self.monkeys[i];
        while self.worry_levels[i].len() > 0 {
            self.processed_items[i] += 1;
            let mut level = self.worry_levels[i].pop().unwrap();
            level = monkey.operation.execute(level) % self.modulus;
            level /= self.divide_by;
            let throw_at = monkey.test.eval(level);
            self.worry_levels[throw_at].push(level);
        }
    }

    fn play_round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.play_monkey(i);
        }
    }
}

#[derive(Debug, Default)]
pub struct AoCDay11 {
    monkeys: Vec<Monkey>
}

impl AoCDay11 {
    fn solve(&self, rounds: usize, divide_by: u64) -> String {
        let mut state = State::new(&self.monkeys, divide_by);
        for _ in 0..rounds {
            state.play_round();
        }
        state.processed_items.sort();
        state.processed_items.reverse();
        (state.processed_items[0] * state.processed_items[1]).to_string()
    }
}

impl AoCProblem for AoCDay11 {
    fn parse_line(&mut self, line: String) {
        if line.trim().len() == 0 {
            return
        } else if line.starts_with("Monkey") {
            self.monkeys.push(Monkey {
                operation: Operation::Add(0),
                starting_items: Vec::new(),
                test: Test {
                    divisible_by: 0,
                    if_false: 0,
                    if_true: 0,
                }
            })
        } else {
            let parts: Vec<&str> = line.split(":").collect();
            let monkey_len = self.monkeys.len();
            let monkey = &mut self.monkeys[monkey_len - 1];
            let last = parts[1].split(" ").last().unwrap().trim();
            match parts[0].trim() {
                "Starting items" => {
                    monkey.starting_items.extend(parts[1].split(",").map(|val| val.trim().parse::<u64>().unwrap()));
                },
                "Operation" => {
                    if parts[1].contains("old * old") {
                        monkey.operation = Operation::Square;
                    } else if parts[1].contains("old * ") {
                        monkey.operation = Operation::Mul(last.parse().unwrap());
                    } else if parts[1].contains("old + ") {
                        monkey.operation = Operation::Add(last.parse().unwrap());
                    } else {
                        panic!("invalid input");
                    }
                },
                "Test" => {
                    monkey.test.divisible_by = last.parse().unwrap();
                },
                "If true" => {
                    monkey.test.if_true = last.parse().unwrap();
                },
                "If false" => {
                    monkey.test.if_false = last.parse().unwrap();
                },
                _ => panic!("invalid input"),
            }
        }
    }

    fn solve_part1(&self) -> String {
        self.solve(20, 3)
    }

    fn solve_part2(&self) -> String {
        self.solve(10_000, 1)
    }
}
