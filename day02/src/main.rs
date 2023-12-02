use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");

    let part1 = part1(input);
    let part2 = part2(input);

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn part1(input: &str) -> usize {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    input
        .lines()
        .map(|x| x.parse::<Game>().unwrap())
        .filter_map(|x| {
            for round in x.rounds {
                if round.blue.unwrap_or(usize::MIN) > max_blue
                    || round.green.unwrap_or(usize::MIN) > max_green
                    || round.red.unwrap_or(usize::MIN) > max_red
                {
                    return None;
                }
            }
            Some(x.id)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<Game>().unwrap())
        .map(|x| {
            x.rounds.iter().filter_map(|y| y.blue).max().unwrap()
                * x.rounds.iter().filter_map(|y| y.red).max().unwrap()
                * x.rounds.iter().filter_map(|y| y.green).max().unwrap()
        })
        .sum()
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

struct Round {
    green: Option<usize>,
    red: Option<usize>,
    blue: Option<usize>,
}

#[derive(Debug)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted_string = s.split(':');

        let game_id = splitted_string
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap();

        let mut game = Game {
            id: game_id,
            rounds: vec![],
        };

        let rounds = splitted_string.next().unwrap().split(';');
        for round in rounds {
            let colors = round.split(',');
            let mut game_round = Round {
                blue: None,
                green: None,
                red: None,
            };

            for color in colors {
                let mut set = color.split_whitespace();
                let amount = set.next().unwrap().parse::<usize>().unwrap();
                let color = set.next().unwrap();

                match color {
                    "green" => game_round.green = Some(amount),
                    "red" => game_round.red = Some(amount),
                    "blue" => game_round.blue = Some(amount),
                    _ => return Err(ParseGameError),
                }
            }

            game.rounds.push(game_round);
        }
        Ok(game)
    }
}
