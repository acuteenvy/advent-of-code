/*
--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look.
The Elves have even given you a map; on it, they've used stars to mark the top fifty
locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations,
you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar;
the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough")
and where they're even sending you ("the sky") and why your map looks mostly blank
("you sure ask a lot of questions") and hang on did you just say the sky
("of course, where do you think snow comes from") when you realize that the Elves are already
loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document
(your puzzle input) has been amended by a very young Elf who was apparently just excited
to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text;
each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining the first digit and the last digit
(in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

--- Part Two ---

Your calculation isn't quite right.
It looks like some of the digits are actually spelled out with letters:
one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
*/

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::result::Result as StdResult;

type BoxErr = Box<dyn Error>;
type Result<T> = StdResult<T, BoxErr>;

fn recover_code1(s: &str) -> Result<usize> {
    let mut iter = s.chars().filter(|x| x.is_ascii_digit());

    let digit1 = iter.next().ok_or::<BoxErr>("input error!".into())?;
    let digit2 = iter.next_back().unwrap_or(digit1);

    Ok(format!("{digit1}{digit2}").parse()?)
}

fn recover_code2(s: &str) -> Result<usize> {
    // Very stupid solution, but it works.
    let s = s
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    recover_code1(&s)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let sum1: usize = input
        .lines()
        .map(recover_code1)
        .collect::<Result<Vec<usize>>>()?
        .into_iter()
        .sum();

    let sum2: usize = input
        .lines()
        .map(recover_code2)
        .collect::<Result<Vec<usize>>>()?
        .into_iter()
        .sum();

    writeln!(
        io::stdout(),
        "(part 1) The sum of all the calibration values is {sum1}\n\
         (part 2) The sum of all the calibration values is {sum2}"
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests are from the puzzle description at the top.

    #[test]
    fn part1() {
        let s = "1abc2";
        assert_eq!(recover_code1(s).unwrap(), 12);

        let s = "pqr3stu8vwx";
        assert_eq!(recover_code1(s).unwrap(), 38);

        let s = "a1b2c3d4e5f";
        assert_eq!(recover_code1(s).unwrap(), 15);

        let s = "treb7uchet";
        assert_eq!(recover_code1(s).unwrap(), 77);
    }

    #[test]
    fn part2() {
        let s = "two1nine";
        assert_eq!(recover_code2(s).unwrap(), 29);

        let s = "eightwothree";
        assert_eq!(recover_code2(s).unwrap(), 83);

        let s = "abcone2threexyz";
        assert_eq!(recover_code2(s).unwrap(), 13);

        let s = "xtwone3four";
        assert_eq!(recover_code2(s).unwrap(), 24);

        let s = "4nineeightseven2";
        assert_eq!(recover_code2(s).unwrap(), 42);

        let s = "zoneight234";
        assert_eq!(recover_code2(s).unwrap(), 14);

        let s = "7pqrstsixteen";
        assert_eq!(recover_code2(s).unwrap(), 76);
    }
}
