/*
--- Day 2: Cube Conundrum ---

You're launched high into the atmosphere!
The apex of your trajectory just barely reaches the surface of a large island floating in the sky.
You gently land in a fluffy pile of leaves.
It's quite cold, but you don't see much snow.
An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow.
He'll be happy to explain the situation, but it's a bit of a walk, so you have some time.
They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
Each time you play this game, he will hide a secret number of cubes of each color in the bag,
and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag,
grab a handful of random cubes, show them to you, and then put them back in the bag.
He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input).
Each game is listed with its ID number (like the 11 in Game 11: ...) followed by
a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put back again).
The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes,
and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained
only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded
with that configuration. However, game 3 would have been impossible because at one point
the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible
because the Elf showed you 15 blue cubes at once.
If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded
with only 12 red cubes, 13 green cubes, and 14 blue cubes.
What is the sum of the IDs of those games?

--- Part Two ---

The Elf says they've stopped producing snow because they aren't getting any water!
He isn't sure why the water stopped; however, he can show you how to get to
the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played,
what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes.
If any color had even one fewer cube, the game would have been impossible.
Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
Game 4 required at least 14 red, 3 green, and 15 blue cubes.
Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.

The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively.
Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present.
What is the sum of the power of these sets?
*/

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::result::Result as StdResult;
use std::str::FromStr;

type BoxErr = Box<dyn Error>;
type Result<T> = StdResult<T, BoxErr>;

#[derive(Clone, Copy)]
struct ElfCubes {
    red: usize,
    green: usize,
    blue: usize,
}

const FULL_SET: ElfCubes = ElfCubes::new(12, 13, 14);

impl ElfCubes {
    pub const fn new(red: usize, green: usize, blue: usize) -> Self {
        ElfCubes { red, green, blue }
    }

    pub fn is_possible(self, full_set: ElfCubes) -> bool {
        self.red <= full_set.red && self.green <= full_set.green && self.blue <= full_set.blue
    }

    pub fn power(self) -> usize {
        self.red * self.green * self.blue
    }
}

const ERR: &str = "input error!";

impl FromStr for ElfCubes {
    type Err = BoxErr;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut cubes = ElfCubes::new(0, 0, 0);

        let spl = s.split(", ");

        for part in spl {
            let mut spl2 = part.split_whitespace();
            let n = spl2.next().ok_or(ERR)?.parse()?;
            let color = spl2.next().ok_or(ERR)?;

            match color {
                "red" => cubes.red = n,
                "green" => cubes.green = n,
                "blue" => cubes.blue = n,
                _ => return Err(ERR.into()),
            }
        }

        Ok(cubes)
    }
}

#[derive(Clone)]
struct Game {
    id: usize,
    cube_sets: Vec<ElfCubes>,
}

impl Game {
    pub fn new(id: usize, cube_sets: Vec<ElfCubes>) -> Self {
        Self { id, cube_sets }
    }

    pub fn is_possible(&self, full_set: ElfCubes) -> bool {
        self.cube_sets.iter().all(|x| x.is_possible(full_set))
    }

    pub fn minimal_set(&self) -> ElfCubes {
        let max_red = self.cube_sets.iter().map(|x| x.red).max().unwrap();
        let max_green = self.cube_sets.iter().map(|x| x.green).max().unwrap();
        let max_blue = self.cube_sets.iter().map(|x| x.blue).max().unwrap();

        ElfCubes::new(max_red, max_green, max_blue)
    }
}

impl FromStr for Game {
    type Err = BoxErr;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut spl = s.split(": ");
        let id = spl
            .next()
            .ok_or(ERR)?
            .split_whitespace()
            .next_back()
            .ok_or(ERR)?
            .parse()?;

        let vec = spl
            .next()
            .ok_or(ERR)?
            .split("; ")
            .map(|x| x.parse())
            .collect::<Result<Vec<ElfCubes>>>()?;

        if vec.is_empty() {
            return Err(ERR.into());
        }

        Ok(Game::new(id, vec))
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut games = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Game>>>()?;

    let games2 = games.clone();
    games.retain(|g| g.is_possible(FULL_SET));
    let sum1: usize = games.into_iter().map(|x| x.id).sum();
    let sum2: usize = games2.into_iter().map(|g| g.minimal_set().power()).sum();

    writeln!(
        io::stdout(),
        "The sum of the IDs of possible games is {sum1}\n\
         The sum of the power of minimal sets is {sum2}"
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests are from the puzzle description at the top.

    #[test]
    fn part1() {
        let g1 = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        let g2 = Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
            .unwrap();
        let g3 = Game::from_str(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        )
        .unwrap();
        let g4 = Game::from_str(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        )
        .unwrap();
        let g5 = Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();

        assert!(g1.is_possible(FULL_SET));
        assert!(g2.is_possible(FULL_SET));
        assert!(!g3.is_possible(FULL_SET));
        assert!(!g4.is_possible(FULL_SET));
        assert!(g5.is_possible(FULL_SET));

        assert_eq!(g1.id + g2.id + g3.id + g4.id + g5.id, 15);
    }

    #[test]
    fn part2() {
        let g1 = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        let g2 = Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
            .unwrap();
        let g3 = Game::from_str(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        )
        .unwrap();
        let g4 = Game::from_str(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        )
        .unwrap();
        let g5 = Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();

        assert_eq!(g1.minimal_set().power(), 48);
        assert_eq!(g2.minimal_set().power(), 12);
        assert_eq!(g3.minimal_set().power(), 1560);
        assert_eq!(g4.minimal_set().power(), 630);
        assert_eq!(g5.minimal_set().power(), 36);
    }
}
