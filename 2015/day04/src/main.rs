/*
--- Day 4: The Ideal Stocking Stuffer ---

Santa needs help mining some AdventCoins (very similar to bitcoins)
to use as gifts for all the economically forward-thinking little girls and boys.

To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes.
The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a number in decimal.
To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.

For example:
    If your secret key is abcdef, the answer is 609043,
    because the MD5 hash of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
    If your secret key is pqrstuv,the lowest number it combines with to make an MD5 hash starting with five zeroes is 1048970;
    that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....

--- Part Two ---

Now find one that starts with six zeroes.
*/

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::result::Result as StdResult;

type Result<T> = StdResult<T, Box<dyn Error>>;

fn find_lowest_n(input: &str, n_zeroes: usize) -> usize {
    let mut n = 1;
    let zeroes = "0".repeat(n_zeroes);

    loop {
        let md5_input = format!("{input}{n}");
        let digest = md5::compute(md5_input);

        if format!("{digest:x}").starts_with(&zeroes) {
            break n;
        }

        n += 1;
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let n5 = find_lowest_n(&input, 5);
    let n6 = find_lowest_n(&input, 6);

    writeln!(
        io::stdout(),
        "Lowest possible n for the MD5 hash to start with 5 zeroes: {n5}\n\
         Lowest possible n for the MD5 hash to start with 6 zeroes: {n6}"
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests are from the puzzle description at the top.

    #[test]
    fn part1() {
        let n = find_lowest_n("abcdef", 5);
        assert_eq!(n, 609043);

        let n = find_lowest_n("pqrstuv", 5);
        assert_eq!(n, 1048970);
    }
}
