use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut ret = 0;
        for line in reader.lines() {
            let line = line?.replace('\u{FEFF}', "");
            let int_parts = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let mut is_ascending = int_parts[0] < int_parts[1];
            let mut valid = true;

            for window in int_parts.windows(2)
            {
                let (prev, current) = (window[0], window[1]);
                if prev.abs_diff(current) > 3
                    || prev == current ||
                    (is_ascending && prev > current)
                    || (!is_ascending && prev < current)
                {
                    valid = false;
                    break;
                }
            }

            if valid {
                ret += 1;
            }
        }
        Ok(ret)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
