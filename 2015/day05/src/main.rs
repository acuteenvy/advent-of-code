/*
--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

    It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

For example:
    ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...),
    double letter (...dd...), and none of the disallowed substrings.

    aaa is nice because it has at least three vowels and a double letter,
    even though the letters used by different rules overlap.

    jchzalrnumimnmhp is naughty because it has no double letter.
    haegwjzuvuyypxyu is naughty because it contains the string xy.
    dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?

--- Part Two ---

Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice.
None of the old rules apply, as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:
    It contains a pair of any two letters that appears at least twice in the string without overlapping,
    like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

For example:
    qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj)
    and a letter that repeats with exactly one letter between them (zxz).

    xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between,
    even though the letters used by each rule overlap.

    uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
    ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

How many strings are nice under these new rules?
*/

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::result::Result as StdResult;

type Result<T> = StdResult<T, Box<dyn Error>>;

fn is_nice1(s: &str) -> bool {
    let chr_vec: Vec<char> = s.chars().collect();

    let three_vowels = {
        let mut three_vowels = false;
        let mut n_vowels = 0;

        for c in &chr_vec {
            if matches!(c, 'a' | 'e' | 'i' | 'o' | 'u') {
                n_vowels += 1;
                if n_vowels >= 3 {
                    three_vowels = true;
                    break;
                }
            }
        }

        three_vowels
    };

    let one_letter_twice = chr_vec.windows(2).any(|win| win[0] == win[1]);

    let no_illegal_strings = chr_vec
        .windows(2)
        .all(|win| !matches!(win, ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y']));

    three_vowels && one_letter_twice && no_illegal_strings
}

fn is_nice2(s: &str) -> bool {
    // todo
    false
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut n_nice1: usize = 0;
    let mut n_nice2: usize = 0;

    for line in input.lines() {
        if is_nice1(line) {
            n_nice1 += 1;
        }
        if is_nice2(line) {
            n_nice2 += 1;
        }
    }

    writeln!(
        io::stdout(),
        "There are {n_nice1} nice string(s) according to the old rules!\n\
         There are {n_nice2} nice string(s) according to the new rules!"
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests are from the puzzle description at the top.

    #[test]
    fn part1() {
        assert!(is_nice1("ugknbfddgicrmopn"));
        assert!(is_nice1("aaa"));

        assert!(!is_nice1("jchzalrnumimnmhp"));
        assert!(!is_nice1("haegwjzuvuyypxyu"));
        assert!(!is_nice1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));

        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
    }
}
