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

const TEST_EDGE_CASES: &str = "\
48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20
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

            if matches!(is_vec_valid(&int_parts), VecState::Valid) {
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut ret = 0;
        for line in reader.lines() {
            let line = line?.replace('\u{FEFF}', "");
            let int_parts = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let vec_status = is_vec_valid(&int_parts);

            match vec_status {
                VecState::Valid => {
                    ret += 1;
                }
                VecState::Invalid(prev, curr) => {
                    // index 0 removal has to be handled in a better way for sure
                    // also there might be optimization to be done depending on error (asc, desc, eq)
                    // here we remove prev, curr, then 0 but depending on why it's invalid there might be better orders
                    if validate_vec_after_removal(&int_parts, prev)
                        || validate_vec_after_removal(&int_parts, curr)
                        || validate_vec_after_removal(&int_parts, 0)
                    {
                        ret += 1
                    }
                }
            }
        }
        Ok(ret)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(10, part2(BufReader::new(TEST_EDGE_CASES.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    enum VecState {
        Valid,
        Invalid(usize, usize),
    }

    fn is_vec_valid(int_parts: &Vec<u32>) -> VecState {
        for i in 1..int_parts.len() {
            let is_ascending = int_parts[0] < int_parts[1];
            let (prev, current) = (int_parts[i - 1], int_parts[i]);

            if prev.abs_diff(current) > 3
                || prev == current
                || (is_ascending && prev > current)
                || (!is_ascending && prev < current)
            {
                return VecState::Invalid(i - 1, i);
            }
        }
        VecState::Valid
    }

    fn validate_vec_after_removal(int_parts: &Vec<u32>, index_to_remove: usize) -> bool {
        let mut int_parts_copy = int_parts.clone();
        int_parts_copy.remove(index_to_remove);
        matches!(is_vec_valid(&int_parts_copy), VecState::Valid)
    }

    Ok(())
}
