// Cubes are red, green, or blue
//
// Each play, hide a secret number of cubes of each color, find out info about # of cubes.
//
// Given a random number of cubes from the bag.
// They are either possible or impossible, given the true # of cubes.

enum Color {
    Red,
    Blue,
    Green,
}

struct Block {
    color: Color,
    qty: u32,
}

struct PartOne(Vec<Block>);

struct MyResult(bool);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    }
}
