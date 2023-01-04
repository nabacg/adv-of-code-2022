use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_plain_code(c: &str) -> Result<Shape, &str> {
        match c {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err("invalid plain code, only A, B and C are supported!"),
        }
    }

    pub fn from_secret_code(c: &str) -> Result<Shape, &str> {
        match c {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err("invalid secret code, only X, Y and Z are supported!"),
        }
    }

    pub fn from_expected_result(r: &GameResult, opponent_move: &Shape) -> Result<Shape, String> {
        let simulated_game = vec![Shape::Rock, Shape::Paper, Shape::Scissors]
            .iter()
            .map(|p2| Game {
                player_1: *opponent_move,
                player_2: *p2,
            })
            .filter(|g| &g.player_2_result() == r)
            .take(1)
            .next();
        if let Some(matching_game) = simulated_game {
            Ok(matching_game.player_2)
        } else {
            Err("Couldn't simulate a expected move to match oppenent_move and produce expected game_result".to_string())
            //, opponent_move, r).as_str())
        }
    }

    pub fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GameResult {
    Lose,
    Draw,
    Win,
}

impl GameResult {
    pub fn score(&self) -> i32 {
        match self {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }

    pub fn from_secret_code(c: &str) -> Result<GameResult, &str> {
        match c {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err("invalid secret result code, only X, Y and Z are supported!"),
        }
    }
}

pub struct Game {
    player_1: Shape,
    player_2: Shape,
}

impl Game {
    pub fn from_input(line: &str) -> Result<Game, String> {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        match parts[..] {
            [left, right] => {
                let l_hand =   Shape::from_plain_code(left)?;
                // let r_hand = Shape::from_secret_code(right)?;
                let expected_result = GameResult::from_secret_code(right)?;
                let r_hand = Shape::from_expected_result(&expected_result, &l_hand)?;
                Ok(Game{
                    player_1: l_hand,
                    player_2: r_hand,
                })
            },
            _ => Err(format!("Invalid inputs found, expected 2 whitespace separated single letter codes, got: {}", line))
        }
    }
    pub fn player_2_result(&self) -> GameResult {
        match (&self.player_1, &self.player_2) {
            (Shape::Rock, Shape::Paper) => GameResult::Win,
            (Shape::Paper, Shape::Scissors) => GameResult::Win,
            (Shape::Scissors, Shape::Rock) => GameResult::Win,
            (Shape::Rock, Shape::Rock) => GameResult::Draw,
            (Shape::Paper, Shape::Paper) => GameResult::Draw,
            (Shape::Scissors, Shape::Scissors) => GameResult::Draw,
            (Shape::Rock, Shape::Scissors) => GameResult::Lose,
            (Shape::Scissors, Shape::Paper) => GameResult::Lose,
            (Shape::Paper, Shape::Rock) => GameResult::Lose,
        }
    }

    pub fn player_2_score(&self) -> i32 {
        self.player_2_result().score() + self.player_2.score()
    }
}

pub fn result(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let games: Result<Vec<Game>, String> = lines.iter().map(|l| Game::from_input(l)).collect();
    match games {
        Ok(games) => {
            let scores: i32 = games.iter().map(|g| g.player_2_score()).sum();
            println!("result is: {}", scores);
            Ok(())
        }
        Err(e) => Err(format!("error processing input: {}", e).into())
    }
}
// DAY 2