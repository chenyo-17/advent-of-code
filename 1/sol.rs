use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_left_right_arrays(file: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left_arr, mut right_arr) = (vec![], vec![]);
    let file = File::open(file).unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let numbers = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();
        left_arr.push(numbers[0]);
        right_arr.push(numbers[1]);
    }
    (left_arr, right_arr)
}

fn main() -> io::Result<()> {
    // let (mut left_arr, mut right_arr) = read_left_right_arrays("1/sample.txt");
    // left_arr.sort();
    // right_arr.sort();
    // let sol_1 = left_arr
    //     .iter()
    //     .enumerate()
    //     .fold(0, |acc, (i, left)| acc + (left - right_arr[i]).abs());
    // print!("sol 1: {sol_1}");
    //
    let (left_arr, right_arr) = read_left_right_arrays("1/input.txt");
    let mut right_freq = HashMap::new();
    right_arr.iter().for_each(|right| {
        right_freq.entry(right).and_modify(|r| *r += 1).or_insert(1);
    });
    let sol_2 = left_arr.iter().fold(0, |acc, left| {
        acc + left * right_freq.get(left).unwrap_or(&0)
    });
    print!("sol 2: {sol_2}");
    Ok(())
}
