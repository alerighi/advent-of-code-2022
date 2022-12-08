use crate::problem::AoCProblem;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    const MOVE_BEATS: [(Move, Move); 3] = [
        (Move::Rock, Move::Scissors),
        (Move::Paper, Move::Rock),
        (Move::Scissors, Move::Paper),
    ];

    fn parse_player_1(c: char) -> Self {
        match c {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn parse_player_2(c: char) -> Self {
        match c {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        Self::MOVE_BEATS.iter().find(|(m1, _m2)| m1 == self).unwrap().1
    }

    fn looses(&self) -> Self {
        Self::MOVE_BEATS.iter().find(|(_m1, m2)| m2 == self).unwrap().0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Player1Win,
    Player2Win,
    Draw,
}

impl Outcome {
    fn parse(c: char) -> Self {
        match c {
            'X' => Outcome::Player1Win,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Player2Win,
            _ => unreachable!(),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Player1Win => 0,
            Outcome::Draw => 3,
            Outcome::Player2Win => 6,
        }
    }
}

#[derive(Debug)]
struct Game {
    player_1_move: Option<Move>,
    player_2_move: Option<Move>,
}

impl Game {
    fn new() -> Self {
        Self {
            player_1_move: None,
            player_2_move: None,
        }
    }

    fn play_player_1(&mut self, mov: Move) {
        self.player_1_move = Some(mov);
    }

    fn play_player_2(&mut self, mov: Move) {
        self.player_2_move = Some(mov);
    }

    fn force_outcome(&mut self, outcome: Outcome) {
        if let Some(mov) = self.player_1_move {
            self.player_2_move = match outcome {
                // to draw, simply to the same move
                Outcome::Draw => Some(mov),
                // for the player one to win, do the move that is beated
                Outcome::Player1Win => Some(mov.beats()),
                // otherwise, do the other move
                Outcome::Player2Win => Some(mov.looses()),
            }
        }
    }

    fn outcome(&self) -> Option<Outcome> {
        if let (Some(m1), Some(m2)) = (self.player_1_move, self.player_2_move) {
            if m1 == m2 {
                Some(Outcome::Draw)
            } else if m1.beats() == m2 {
                Some(Outcome::Player1Win)
            } else {
                Some(Outcome::Player2Win)
            }
        } else {
            None
        }
    }

    fn score(&self) -> Option<u32> {
        if let (Some(m2), Some(outcome)) = (self.player_2_move, self.outcome()) {
            Some(m2.score() + outcome.score())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct AoCDay2 {
    moves: Vec<(char, char)>
}

impl Default for AoCDay2 {
    fn default() -> Self {
        Self {
            moves: Vec::new()
        }
    }
}

impl AoCProblem for AoCDay2 {
    fn parse_line(&mut self, line: String) {
        let chars: Vec<char> = line.chars().collect();
        self.moves.push((chars[0], chars[2]));
    }

    fn solve_part1(&self) -> String {
        let mut score = 0;
        for (pl_1, pl_2) in self.moves.iter() {
            let mut game = Game::new();
            game.play_player_1(Move::parse_player_1(*pl_1));
            game.play_player_2(Move::parse_player_2(*pl_2));

            score += game.score().unwrap();
        }

        score.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut score = 0;
        for (pl_1, out) in self.moves.iter() {
            let mut game = Game::new();
            game.play_player_1(Move::parse_player_1(*pl_1));
            game.force_outcome(Outcome::parse(*out));

            score += game.score().unwrap();
        }

        score.to_string()
    }
}
