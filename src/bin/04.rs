use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

const TEST_2: &str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
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

                    // compute horizontal
                    let mut potential_xmases = "X".to_string();
                    if can_check_left {
                        potential_xmases
                            .insert_str(0, &text_as_chars[i][j - 3..j].iter().collect::<String>());
                    }
                    if can_check_right {
                        potential_xmases
                            .push_str(&text_as_chars[i][j + 1..=j + 3].iter().collect::<String>());
                    }
                    ret += number_of_xmases(potential_xmases);

                    // compute vertical
                    potential_xmases = "X".to_string();
                    if can_check_up {
                        let potential_xmas: String = text_as_chars[i - 3..i]
                            .iter()
                            .filter_map(|line| line.get(j))
                            .collect();
                        potential_xmases.insert_str(0, &potential_xmas);
                    }
                    if can_check_down {
                        let potential_xmas: String = text_as_chars[i + 1..=i + 3]
                            .iter()
                            .filter_map(|line| line.get(j))
                            .collect();
                        potential_xmases.push_str(&potential_xmas);
                    }
                    ret += number_of_xmases(potential_xmases);

                    // compute diagonal left to right
                    potential_xmases = "X".to_string();
                    if can_check_up && can_check_left {
                        for k in 1..=3 {
                            potential_xmases.insert(0, text_as_chars[i - k][j - k]);
                        }
                    }
                    if can_check_down && can_check_right {
                        for k in 1..=3 {
                            potential_xmases.push(text_as_chars[i + k][j + k]);
                        }
                    }
                    ret += number_of_xmases(potential_xmases);

                    // compute diagonal right to left
                    potential_xmases = "X".to_string();
                    if can_check_up && can_check_right {
                        for k in 1..=3 {
                            potential_xmases.insert(0, text_as_chars[i - k][j + k]);
                        }
                    }
                    if can_check_down && can_check_left {
                        for k in 1..=3 {
                            potential_xmases.push(text_as_chars[i + k][j - k]);
                        }
                    }
                    ret += number_of_xmases(potential_xmases);
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

        // for some reason part 2 is easier
        // start indexes from 1, making bound checking useless
        for i in 1..text_as_chars.len() - 1 {
            for j in 1..text_as_chars[i].len() - 1 {
                if text_as_chars[i][j] == 'A' {
                    let (top_left, top_right, bottom_left, bottom_right) = (
                        text_as_chars[i - 1][j - 1],
                        text_as_chars[i - 1][j + 1],
                        text_as_chars[i + 1][j - 1],
                        text_as_chars[i + 1][j + 1],
                    );
                    // disgusting but fast
                    if ((top_left, bottom_right) == ('M', 'S')
                        || (top_left, bottom_right) == ('S', 'M'))
                        && ((top_right, bottom_left) == ('M', 'S')
                            || (top_right, bottom_left) == ('S', 'M'))
                    {
                        ret += 1;
                    }
                }
            }
        }
        Ok(ret)
    }

    assert_eq!(9, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

fn number_of_xmases(word: String) -> usize {
    let mut ret = 0;
    if word.contains("XMAS") {
        ret += 1;
    }
    if word.contains("SAMX") {
        ret += 1;
    }
    ret
}
