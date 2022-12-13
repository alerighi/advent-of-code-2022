use std::cmp::Ordering;

use crate::problem::AoCProblem;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Integer(u32),
    List(Vec<Item>),
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match self {
            Item::Integer(val) => val.to_string(),
            Item::List(values) => {
                let mut result = String::new();
                result += "[";
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        result += ",";
                    }
                    result += &value.to_string();
                }
                result += "]";
                result
            }
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::List(_), Item::Integer(i2)) => self.cmp(&Item::List(vec![Item::Integer(*i2)])),
            (Item::Integer(i1), Item::List(_)) => Item::List(vec![Item::Integer(*i1)]).cmp(other),
            (Item::Integer(i1), Item::Integer(i2)) => i1.cmp(i2),
            (Item::List(l1), Item::List(l2)) => {
                for i in 0..l1.len().min(l2.len()) {
                    match l1[i].cmp(&l2[i]) {
                        Ordering::Equal => {},
                        ordering => return ordering,
                    }
                }
                l1.len().cmp(&l2.len())
            },
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default)]
pub struct AoCDay13 {
    packets: Vec<Item>
}

enum Token {
    LParen,
    RParen,
    Comma,
    Integer(u32),
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_number: Option<u32> = None;

    for c in line.chars() {
        match c {
            '0'..='9' => current_number = Some(current_number.unwrap_or(0) * 10 + c.to_digit(10).unwrap()),
            _ => {
                if let Some(value) = current_number {
                    tokens.push(Token::Integer(value));
                    current_number = None;
                }
                match c {
                    '[' => tokens.push(Token::LParen),
                    ']' => tokens.push(Token::RParen),
                    ',' => tokens.push(Token::Comma),
                    _ => panic!("invalid token: {}", c),
                }

            }
        }
    }
    tokens
}

fn parse(line: &str) -> Item {
    let mut list: Vec<Vec<Item>> = Vec::new();
    for tok in tokenize(line) {
        match tok {
            Token::LParen => {
                list.push(Vec::new());
            },
            Token::RParen => {
                let value = list.pop().unwrap();
                if let Some(top) = list.last_mut() {
                    top.push(Item::List(value));
                } else {
                    return Item::List(value);
                }
            },
            Token::Comma => {},
            Token::Integer(val) => {
                list.last_mut().unwrap().push(Item::Integer(val));
            },
        }
    }

    panic!("invalid input");
}

impl AoCProblem for AoCDay13 {
    fn parse_line(&mut self, line: String) {
        if line.len() > 0 {
            self.packets.push(parse(&line));
        }
    }

    fn solve_part1(&self) -> String {
        let mut res = 0;
        for i in 0..(self.packets.len() / 2) {
            let a = &self.packets[i * 2];
            let b = &self.packets[i * 2 + 1];
            if a <= b {
                res += i + 1;
            }
        }
        res.to_string()
    }

    fn solve_part2(&self) -> String {
        let div1 = Item::List(vec![Item::List(vec![Item::Integer(2)])]);
        let div2 = Item::List(vec![Item::List(vec![Item::Integer(6)])]);
        let mut packets = self.packets.clone();
        packets.push(div1.clone());
        packets.push(div2.clone());
        packets.sort();

        let div1_i = packets.binary_search(&div1).unwrap() + 1;
        let div2_i = packets.binary_search(&div2).unwrap() + 1;

        (div1_i * div2_i).to_string()
    }
}
