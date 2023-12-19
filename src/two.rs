// Cubes are red, green, or blue
//
// Each play, hide a secret number of cubes of each color, find out info about # of cubes.
//
// Given a random number of cubes from the bag.
// They are either possible or impossible, given the true # of cubes.

// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    multi::{separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};

fn parse_block(i: &str) -> IResult<&str, Block> {
    let (i, numstr) = digit1(i)?;
    let (i, _) = space0(i)?;
    let (i, blockstr) = alt((tag("red"), tag("blue"), tag("green")))(i)?;
    let num: u32 = numstr.parse().expect("Invalid number");
    let block = match blockstr {
        "red" => Block::Red(num),
        "blue" => Block::Blue(num),
        "green" => Block::Green(num),
        _ => unreachable!("Block pattern should be exhaustive"),
    };
    Ok((i, block))
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    let (i, game_id) = delimited(tag("Game "), digit1, tag(": "))(i)?;
    let game_id: u32 = game_id.parse().expect("Game id not number");
    let parse_blocks = separated_list1(tag(", "), parse_block);
    let (i, draws): (&str, Vec<Vec<Block>>) = separated_list0(tag("; "), parse_blocks)(i)?;
    Ok((i, Game { game_id, draws }))
}

fn parse_games(i: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(tag("\n"), parse_game)(i)
}

#[derive(PartialEq, Eq, Debug)]
enum Block {
    Red(u32),
    Blue(u32),
    Green(u32),
}

#[derive(Debug)]
struct Game {
    game_id: u32,
    draws: Vec<Vec<Block>>,
}

impl Game {
    fn is_valid(&self) -> bool {
        let (R, G, B) = (12, 13, 14);

        let mut result = false;
        for draw in &self.draws {
            let (mut r, mut g, mut b) = (0, 0, 0);
            for block in draw {
                match block {
                    Block::Red(x) => r = r + x,
                    Block::Green(x) => g = g + x,
                    Block::Blue(x) => b = b + x,
                };
            }

            if !(r <= R && g <= G && b <= B) {
                return false;
            }
        }
        true
    }
}

struct PartOne(Vec<Game>);

struct PartTwo(Vec<Block>);

struct MyResult(u32);

impl From<PartOne> for MyResult {
    fn from(value: PartOne) -> Self {
        let PartOne(games) = value;
        Self(
            games
                .iter()
                .filter(|g| g.is_valid())
                .fold(0, |acc, g| acc + g.game_id),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_parse_block() {
        let (_, a) = parse_block("3 blue").unwrap();
        let (_, b) = parse_block("40000 red").unwrap();
        let (_, c) = parse_block("0 green").unwrap();

        let x = Block::Blue(3);
        let y = Block::Red(40000);
        let z = Block::Green(0);

        assert_eq!(a, x);
        assert_eq!(b, y);
        assert_eq!(c, z);
    }

    #[test]
    fn test_parse_game() {
        let s = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = Game {
            game_id: 2,
            draws: vec![
                vec![Block::Blue(1), Block::Green(2)],
                vec![Block::Green(3), Block::Blue(4), Block::Red(1)],
                vec![Block::Green(1), Block::Blue(1)],
            ],
        };
        let (i, actual) = parse_game(s).unwrap();
        assert_eq!(expected.game_id, actual.game_id);
        println!("{actual:?} {i}");

        for (ix, draw) in expected.draws.iter().enumerate() {
            for (jx, expected_block) in draw.iter().enumerate() {
                assert_eq!(expected_block, &actual.draws[ix][jx]);
            }
        }
    }

    #[test]
    fn test_part_one_known() {
        let games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let (_, games) = parse_games(games).unwrap();
        let result = MyResult::from(PartOne(games));
        assert_eq!(8, result.0);
    }

    #[test]
    fn test_part_one() {
        let s = read_to_string("resources/2.txt").unwrap();
        let (_, games) = parse_games(&s).unwrap();
        let result = MyResult::from(PartOne(games));
        assert_eq!(8, result.0);
    }
}
