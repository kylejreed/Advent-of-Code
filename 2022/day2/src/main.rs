use std::{str::FromStr, string::ParseError, fs};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};

#[derive(Display, Debug, PartialEq, EnumIter, Clone, Copy)]
enum RockPaperScizzor {
  Rock,
  Paper,
  Scizzor,
  Win,
  Lose,
  Draw
}

impl RockPaperScizzor {
    fn value(&self) -> i32 {
      match *self {
        RockPaperScizzor::Rock => 1,
        RockPaperScizzor::Paper => 2,
        RockPaperScizzor::Scizzor => 3,
        _ => 0
      }
    }

    fn beats(&self, other: &RockPaperScizzor) -> bool {
      match *self {
        RockPaperScizzor::Rock => *other == RockPaperScizzor::Scizzor,
        RockPaperScizzor::Paper => *other == RockPaperScizzor::Rock,
        RockPaperScizzor::Scizzor => *other == RockPaperScizzor::Paper,
        _ => panic!("Why am i here")
      }
    }

    fn convert(&self, other: &RockPaperScizzor) -> RockPaperScizzor {
      match *self {
        RockPaperScizzor::Win => RockPaperScizzor::iter().find(|v| v.beats(other)).unwrap(),
        RockPaperScizzor::Lose => RockPaperScizzor::iter().find(|v| v != other && !v.beats(other)).unwrap(),
        RockPaperScizzor::Draw => other.clone(),
        _ => self.clone()
      }
    }
}

impl FromStr for RockPaperScizzor {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "A" => Ok(RockPaperScizzor::Rock),
      "B" => Ok(RockPaperScizzor::Paper),
      "C" => Ok(RockPaperScizzor::Scizzor),
      "X" => Ok(RockPaperScizzor::Lose),
      "Y" => Ok(RockPaperScizzor::Draw),
      "Z" => Ok(RockPaperScizzor::Win),
      _ => todo!()
    }
  }
}

struct Rounds {
  rounds: Vec<Round>
}

impl FromStr for Rounds {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
      let result = s.split('\n').map(|line| {
        Round::from_str(line).unwrap()
      }).collect::<Vec<Round>>();
      Ok(Rounds { rounds: result })
  }
}

struct Round {
  player1: RockPaperScizzor,
  player2: RockPaperScizzor
}

impl Round {
  fn player2_score(&self) -> i32 {
    self.calc_score(&self.player2, &self.player1)
  }

  fn calc_score(&self, s1: &RockPaperScizzor, s2: &RockPaperScizzor) -> i32 {
    let v1 = s1.convert(s2);
    let v2 = s2.convert(s1);
    let mut score = v1.value();
    if v1.beats(&v2) {
      score += 6
    } else if  v1 == v2 {
      score += 3
    }

    score
  }
}

impl FromStr for Round {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
      let (l, r) = s.split_once(' ').unwrap();
      Ok(Round {
        player1: RockPaperScizzor::from_str(l).unwrap(),
        player2: RockPaperScizzor::from_str(r).unwrap()
      })
  }
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    let rounds = Rounds::from_str(&file).unwrap();
    let score: i32 = rounds.rounds.into_iter().map(|r| r.player2_score()).sum();
    println!("Score: {score}");
}
