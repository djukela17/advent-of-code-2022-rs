use crate::day02::Shape::Scissors;

pub fn part_one(input: String) -> i32 {
    let rounds = parse_input(input);

    let (_enemy_score, my_score): (i32, i32) = rounds
        .into_iter()
        .map(|round| -> (i32, i32) { round.calculate_points() })
        .fold(
            (0, 0),
            |total_scores: (i32, i32), round_scores: (i32, i32)| -> (i32, i32) {
                (
                    total_scores.0 + round_scores.0,
                    total_scores.1 + round_scores.1,
                )
            },
        );

    my_score
}

pub fn part_two(input: String) -> i32 {
    let rounds = parse_input_v2(input);

    let (_enemy_score, my_score): (i32, i32) = rounds
        .into_iter()
        .map(|round| -> (i32, i32) { round.calculate_points() })
        .fold(
            (0, 0),
            |total_scores: (i32, i32), round_scores: (i32, i32)| -> (i32, i32) {
                (
                    total_scores.0 + round_scores.0,
                    total_scores.1 + round_scores.1,
                )
            },
        );

    my_score
}

fn parse_input(input: String) -> Vec<Round> {
    input
        .lines()
        .filter(|line| *line != "")
        .map(|line| -> Round {
            let parts: Vec<&str> = line.split(" ").collect();

            Round::new(Shape::new(parts[0]), Shape::new(parts[1]))
        })
        .collect()
}

fn parse_input_v2(input: String) -> Vec<Round> {
    input
        .lines()
        .filter(|line| *line != "")
        .map(|line| -> Round {
            let parts: Vec<&str> = line.split(" ").collect();

            let enemy_shape = Shape::new(parts[0]);

            let my_shape = match parts[1] {
                "X" => enemy_shape.wins_against(),
                "Y" => enemy_shape.clone(),
                "Z" => enemy_shape.loses_against(),
                _ => panic!("invalid"),
            };

            Round::new(enemy_shape, my_shape)
        })
        .collect()
}

#[derive(Debug)]
struct Round {
    enemy_play: Shape,
    my_play: Shape,
}

impl Round {
    pub fn new(enemy_play: Shape, my_play: Shape) -> Round {
        Round {
            enemy_play,
            my_play,
        }
    }

    pub fn calculate_points(&self) -> (i32, i32) {
        let enemy_shape_score = self.enemy_play.score();
        let my_shape_score = self.my_play.score();

        let (lhs_score, rhs_score) = self.enemy_play.compare(&self.my_play);

        (enemy_shape_score + lhs_score, my_shape_score + rhs_score)
    }
}

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn new(shape: &str) -> Shape {
        match shape {
            "A" => Shape::Rock,
            "X" => Shape::Rock,
            "B" => Shape::Paper,
            "Y" => Shape::Paper,
            "C" => Shape::Scissors,
            "Z" => Shape::Scissors,
            _ => panic!("invalid input shape"),
        }
    }

    pub fn compare(&self, other: &Self) -> (i32, i32) {
        match self {
            Shape::Rock => match other {
                Shape::Rock => (3, 3),
                Shape::Paper => (0, 6),
                Shape::Scissors => (6, 0),
            },
            Shape::Paper => match other {
                Shape::Rock => (6, 0),
                Shape::Paper => (3, 3),
                Shape::Scissors => (0, 6),
            },
            Shape::Scissors => match other {
                Shape::Rock => (0, 6),
                Shape::Paper => (6, 0),
                Shape::Scissors => (3, 3),
            },
        }
    }

    pub fn loses_against(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    pub fn wins_against(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
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
