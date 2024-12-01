use std::collections::hash_map::Entry::Vacant;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let mut left = vec![];
        let mut right = vec![];
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
                left.push(first.replace('\u{FEFF}', "").parse::<u32>()?);
                right.push(second.replace('\u{FEFF}', "").parse::<u32>()?);
            }
        }

        left.sort();
        right.sort();

        let ret = left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| if a < b { b - a } else { a - b })
            .sum();

        Ok(ret)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let mut left = vec![];
        let mut right = vec![];
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
                left.push(first.replace('\u{FEFF}', "").parse::<u32>()?);
                right.push(second.replace('\u{FEFF}', "").parse::<u32>()?);
            }
        }

        let mut ret = 0;
        let mut similarity_hashmap: HashMap<u32, u32> = HashMap::new();
        for value in left {
            if let Vacant(e) = similarity_hashmap.entry(value) {
                let numbers_in_right = right.iter().filter(|&x| *x == value).count() as u32;
                e.insert(numbers_in_right);
                ret += value * similarity_hashmap.get(&value).unwrap();
            } else {
                ret += value * similarity_hashmap.get(&value).unwrap();
            }
        }

        Ok(ret)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
