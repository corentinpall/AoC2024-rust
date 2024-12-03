use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut ret = 0;

        let reg = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;
        let instructions = process_lines(reader, &reg)?;

        for pair in instructions.split_whitespace() {
            ret += parse_mul_instruction(pair)?;
        }
        Ok(ret)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut ret = 0;
        let reg = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))")?;
        let instructions = process_lines(reader, &reg)?;

        let mut is_do = true;
        for instruction in instructions.split_whitespace() {
            match instruction {
                "do()" => {
                    is_do = true;
                }
                "don't()" => {
                    is_do = false;
                }
                _ if is_do => {
                    ret += parse_mul_instruction(instruction)?;
                }
                _ => {}
            }
        }

        Ok(ret)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn process_lines<R: BufRead>(reader: R, regex: &Regex) -> Result<String> {
    let mut result = String::new();
    for line in reader.lines() {
        let line = line?.replace('\u{FEFF}', ""); // Remove BOM
        let clean = regex
            .find_iter(&line)
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        result += " ";
        result += &clean.trim();
    }
    Ok(result)
}

fn parse_mul_instruction(instruction: &str) -> Result<usize> {
    let instruction = &instruction[4..instruction.len() - 1]; // Remove "mul(" and ")"
    let pair = instruction.split(',').collect_vec();
    Ok(pair[0].parse::<usize>()? * pair[1].parse::<usize>()?)
}
