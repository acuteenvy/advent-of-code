/*
--- Day 3: Perfectly Spherical Houses in a Vacuum ---

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole
calls him via radio and tells him where to move next.
Moves are always exactly one house to the north (^), south (v), east (>), or west (<).
After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog,
and so his directions are a little off, and Santa ends up visiting some houses more than once.
How many houses receive at least one present?

For example:
    ">" delivers presents to 2 houses: one at the starting location, and one to the east.
    "^>v<" delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    "^v^v^v^v^v" delivers a bunch of presents to some very lucky children at only 2 houses.

--- Part Two ---

The next year, to speed up the process, Santa creates a robot version of himself,Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the same starting house),
then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:
    "^v" delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    "^>v<" now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    "^v^v^v^v^v" now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
*/

use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::ops;
use std::result::Result as StdResult;

type Result<T> = StdResult<T, Box<dyn Error>>;

#[derive(Clone, Copy)]
enum Move {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(value: char) -> StdResult<Self, Self::Error> {
        match value {
            '^' => Ok(Move::North),
            'v' => Ok(Move::South),
            '>' => Ok(Move::East),
            '<' => Ok(Move::West),
            _ => Err("invalid move!"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl ops::AddAssign<Move> for Point {
    fn add_assign(&mut self, rhs: Move) {
        match rhs {
            Move::North => self.y += 1,
            Move::South => self.y -= 1,
            Move::East => self.x += 1,
            Move::West => self.x -= 1,
        }
    }
}

struct Santa {
    moves: Vec<Move>,
    current_pos: Point,
    houses: HashSet<Point>,
}

impl Santa {
    fn new(moves: &str) -> Self {
        Self {
            moves: moves
                .chars()
                .filter_map(|c| c.try_into().ok())
                .collect::<Vec<Move>>(),
            current_pos: Point::new(0, 0),
            houses: HashSet::new(),
        }
    }

    /// How many houses receive at least one present?
    fn follow_instructions(mut self) -> usize {
        self.houses.insert(self.current_pos);

        for mv in self.moves {
            self.current_pos += mv;
            self.houses.insert(self.current_pos);
        }

        self.houses.len()
    }
}

struct SantaAndHisRobot {
    moves: Vec<Move>,
    santa_pos: Point,
    robot_pos: Point,
    houses: HashSet<Point>,
}

impl SantaAndHisRobot {
    fn new(moves: &str) -> Self {
        Self {
            moves: moves
                .chars()
                .filter_map(|c| c.try_into().ok())
                .collect::<Vec<Move>>(),
            santa_pos: Point::new(0, 0),
            robot_pos: Point::new(0, 0),
            houses: HashSet::new(),
        }
    }

    /// How many houses receive at least one present?
    fn follow_instructions(mut self) -> usize {
        self.houses.insert(self.santa_pos);

        for (idx, mv) in self.moves.into_iter().enumerate() {
            if idx % 2 == 0 {
                self.robot_pos += mv;
                self.houses.insert(self.robot_pos);
            } else {
                self.santa_pos += mv;
                self.houses.insert(self.santa_pos);
            }
        }

        self.houses.len()
    }
}

fn main() -> Result<()> {
    let moves = fs::read_to_string("input.txt")?;

    let n_houses1 = Santa::new(&moves).follow_instructions();
    let n_houses2 = SantaAndHisRobot::new(&moves).follow_instructions();

    writeln!(
        io::stdout(),
        "Santa has delivered presents to {n_houses1} house(s)!\n\
        Santa and his Robot have delivered presents to {n_houses2} house(s)!"
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests are from the puzzle description at the top.

    #[test]
    fn part1() {
        let n_houses = Santa::new(">").follow_instructions();
        assert_eq!(n_houses, 2);

        let n_houses = Santa::new("^>v<").follow_instructions();
        assert_eq!(n_houses, 4);

        let n_houses = Santa::new("^v^v^v^v^v").follow_instructions();
        assert_eq!(n_houses, 2);
    }

    #[test]
    fn part2() {
        let n_houses = SantaAndHisRobot::new(">v").follow_instructions();
        assert_eq!(n_houses, 3);

        let n_houses = SantaAndHisRobot::new("^>v<").follow_instructions();
        assert_eq!(n_houses, 3);

        let n_houses = SantaAndHisRobot::new("^v^v^v^v^v").follow_instructions();
        assert_eq!(n_houses, 11);
    }
}
