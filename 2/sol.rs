use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct LineIterator<T, F> {
    reader: BufReader<File>,
    parser: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> LineIterator<T, F>
where
    F: Fn(&str) -> T,
{
    fn new(file: &str, parser: F) -> Self {
        let file = File::open(file).unwrap();
        LineIterator {
            reader: BufReader::new(file),
            parser,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, F> Iterator for LineIterator<T, F>
where
    F: Fn(&str) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => Some((self.parser)(&line)),
            Err(_) => unreachable!(),
        }
    }
}

fn is_monotonic_within_diff_3(arr: &[i32]) -> bool {
    if arr.len() < 2 {
        return true;
    }
    let diff = arr[1] - arr[0];
    if !(1..=3).contains(&diff.abs()) {
        return false;
    }
    let mut prev_sign = diff.signum();
    arr.windows(2).all(|w| {
        let d = w[1] - w[0];
        let curr_sign = d.signum();
        if curr_sign != prev_sign || !((1..=3).contains(&d.abs())) {
            return false;
        }
        prev_sign = curr_sign;
        true
    })
}

fn is_monotonic_within_diff_3_after_remove_one_element(arr: &[i32]) -> bool {
    if is_monotonic_within_diff_3(&arr) {
        return true;
    }
    (0..arr.len()).any(|i| {
        let mut temp = arr.to_vec();
        temp.remove(i);
        is_monotonic_within_diff_3(&temp)
    })
}

fn main() -> io::Result<()> {
    let line_parser = |line: &str| -> Vec<i32> {
        line.split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect()
    };
    let iter = LineIterator::new("2/input.txt", line_parser);
    // let sol_1 = iter.filter(|arr| is_monotonic_within_diff_3(arr)).count();
    // println!("sol 1: {sol_1}");
    let sol_2 = iter
        .filter(|arr| is_monotonic_within_diff_3_after_remove_one_element(arr))
        .count();
    println!("sol 2: {sol_2}");
    Ok(())
}
