use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut ret = 0;
        let text_as_chars: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.map(|line| line.replace('\u{FEFF}', "").chars().collect()))
            .collect::<Result<_, _>>()?;

        // let's start the horrible nesting
        for i in 0..text_as_chars.len() {
            for j in 0..text_as_chars[i].len() {
                if text_as_chars[i][j] == 'X' {
                    let can_check_left = j >= 3;
                    let can_check_right = j < text_as_chars[i].len() - 3;
                    let can_check_up = i >= 3;
                    let can_check_down = i < text_as_chars.len() - 3;

                    // check left
                    if can_check_left {
                        let potential_xmas = text_as_chars[i][j - 3..=j].iter().collect::<String>();
                        if is_xmas(potential_xmas) {
                            ret += 1;
                        }
                    }

                    // check right
                    if can_check_right {
                        let potential_xmas = text_as_chars[i][j..=j + 3].iter().collect::<String>();
                        if is_xmas(potential_xmas) {
                            ret += 1;
                        }
                    }

                    // check up
                    if can_check_up {
                        let potential_xmas = text_as_chars[i - 3..=i]
                            .iter()
                            .filter_map(|line| line.get(j))
                            .collect();
                        if is_xmas(potential_xmas) {
                            ret += 1;
                        }
                    }

                    // check down
                    if can_check_down {
                        let potential_xmas = text_as_chars[i..=i + 3]
                            .iter()
                            .filter_map(|line| line.get(j))
                            .collect();
                        if is_xmas(potential_xmas) {
                            ret += 1;
                        }
                    }

                    // check up left
                    if can_check_up && can_check_left {
                        let mut potential_xmas = "".to_string();
                        for k in 0..=3 {
                            potential_xmas.push(text_as_chars[i - k][j - k]);
                        }
                        if is_xmas(potential_xmas) {
                            ret += 1;
                        }
                    }

                    // check up right
                    if can_check_up && can_check_right {
                        let mut potential_xmas = "".to_string();
                        for k in 0..=3 {
                            potential_xmas.push(text_as_chars[i - k][j + k]);
                        }
                        if is_xmas(potential_xmas) {
                            ret += 1;
                        }
                    }

                    // check down left
                    if can_check_down && can_check_left {
                        let mut potential_xmas = "".to_string();
                        for k in 0..=3 {
                            potential_xmas.push(text_as_chars[i + k][j - k]);
                        }
                        if is_xmas(potential_xmas) {
                            ret += 1;
                        }
                    }

                    // check down right
                    if can_check_down && can_check_right {
                        let mut potential_xmas = "".to_string();
                        for k in 0..=3 {
                            potential_xmas.push(text_as_chars[i + k][j + k]);
                        }
                        if is_xmas(potential_xmas) {
                            ret += 1;
                        }
                    }
                }
            }
        }
        Ok(ret)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut ret = 0;
        let text_as_chars: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.map(|line| line.replace('\u{FEFF}', "").chars().collect()))
            .collect::<Result<_, _>>()?;

        // for some reason part 2 is easier and cleaner
        for i in 0..text_as_chars.len() {
            for j in 0..text_as_chars[i].len() {
                if text_as_chars[i][j] == 'A' {}
            }
        }
        Ok(ret)
    }

    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn is_xmas(word: String) -> bool {
    word == "XMAS" || word == "SAMX"
}
