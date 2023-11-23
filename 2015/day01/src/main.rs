/*
--- Day 1: Not Quite Lisp ---

Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars,
and he's fresh out! To save Christmas, he needs you to collect fifty stars by December 25th.

Collect stars by helping Santa solve puzzles.
Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first.
Each puzzle grants one star. Good luck!

Here's an easy puzzle to warm you up.

Santa is trying to deliver presents in a large apartment building,
but he can't find the right floor - the directions he got are a little confusing.
He starts on the ground floor (floor 0) and then follows the moves one character at a time.

An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.

The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.

For example:
    "(())" and "()()" both result in floor 0.
    "(((" and "(()(()(" both result in floor 3.
    ""))(((((" also results in floor 3.
    "())" and "))(" both result in floor -1 (the first basement level).
    ")))" and ")())())" both result in floor -3.

To what floor do the moves take Santa?

--- Part Two ---

Now, given the same moves, find the position of the first character that causes him to enter the basement (floor -1).
The first character in the moves has position 1, the second character has position 2, and so on.

For example:
    ")" causes him to enter the basement at character position 1.
    "()())" causes him to enter the basement at character position 5.

What is the position of the character that causes Santa to first enter the basement?
*/

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::result::Result as StdResult;

type Result<T> = StdResult<T, Box<dyn Error>>;

enum Move {
    UpFloor,
    DownFloor,
}

impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(value: char) -> StdResult<Self, Self::Error> {
        match value {
            '(' => Ok(Move::UpFloor),
            ')' => Ok(Move::DownFloor),
            _ => Err("invalid move!"),
        }
    }
}

struct Santa {
    moves: Vec<Move>,
    current_floor: i32,
}

impl Santa {
    fn new(moves: &str) -> Self {
        Self {
            current_floor: 0,
            moves: moves
                .chars()
                .filter_map(|c| c.try_into().ok())
                .collect::<Vec<Move>>(),
        }
    }

    /// On which floor does Santa end up after all moves?
    /// And how many instuctions does it take for Santa to enter the basement?
    fn follow_instructions(mut self) -> (i32, Option<usize>) {
        let mut basement_instruction_pos = None;

        for (idx, mv) in self.moves.iter().enumerate() {
            match mv {
                Move::UpFloor => self.current_floor += 1,
                Move::DownFloor => self.current_floor -= 1,
            }

            if basement_instruction_pos.is_none() && self.current_floor == -1 {
                basement_instruction_pos = Some(idx + 1);
            }
        }

        (self.current_floor, basement_instruction_pos)
    }
}

fn main() -> Result<()> {
    let moves = fs::read_to_string("input.txt")?;

    let (resulting_floor, basement_pos) = Santa::new(&moves).follow_instructions();
    let mut stdout = io::stdout().lock();

    writeln!(stdout, "Santa is on floor {resulting_floor}!")?;

    if let Some(pos) = basement_pos {
        writeln!(
            stdout,
            "Santa has entered the basement after {pos} instruction(s)!"
        )?;
    } else {
        writeln!(stdout, "Santa hasn't entered the basement after all moves!")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests are from the puzzle description at the top.

    fn res_floor(input: &str) -> i32 {
        let (resulting_floor, _) = Santa::new(input).follow_instructions();
        resulting_floor
    }

    fn basement_pos(input: &str) -> usize {
        let (_, basement_pos) = Santa::new(input).follow_instructions();
        basement_pos.unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(res_floor("(())"), 0);
        assert_eq!(res_floor("()()"), 0);
        assert_eq!(res_floor("((("), 3);
        assert_eq!(res_floor("(()(()("), 3);
        assert_eq!(res_floor("))((((("), 3);
        assert_eq!(res_floor("())"), -1);
        assert_eq!(res_floor("))("), -1);
        assert_eq!(res_floor(")))"), -3);
        assert_eq!(res_floor(")())())"), -3);
    }

    #[test]
    fn part2() {
        assert_eq!(basement_pos(")"), 1);
        assert_eq!(basement_pos("()())"), 5);
    }
}
