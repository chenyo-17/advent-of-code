use regex::Regex;
use std::fs;

fn end_index_of_first_match(re: &Regex, text: &str) -> Option<usize> {
    if let Some(mat) = re.find(text) {
        return Some(mat.end());
    }
    None
}

fn main() -> std::io::Result<()> {
    // let text = fs::read_to_string("3/sample.txt").unwrap();
    // let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    // let sol_1 = re.captures_iter(&text).fold(0, |acc, caps| {
    //     let first: i32 = caps[1].parse().unwrap();
    //     let second: i32 = caps[2].parse().unwrap();
    //     acc + first * second
    // });
    // println!("sol 1: {sol_1}");
    //
    let text = fs::read_to_string("3/input.txt").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut index_do_end = 0;
    let mut sol_2 = 0;
    while index_do_end < text.len() {
        let index_dont_end = index_do_end
            + end_index_of_first_match(&re_dont, &text[index_do_end..])
                .unwrap_or(text.len() - index_do_end);
        // println!("index_dont_end: {index_dont_end}");
        sol_2 = re_mul
            .captures_iter(&text[index_do_end..index_dont_end])
            .fold(sol_2, |acc, caps| {
                let first: i32 = caps[1].parse().unwrap();
                let second: i32 = caps[2].parse().unwrap();
                acc + first * second
            });
        if index_dont_end < text.len() {
            index_do_end = index_dont_end
                + end_index_of_first_match(&re_do, &text[index_dont_end..])
                    .unwrap_or(text.len() - index_dont_end);
        } else {
            break;
        }
        // println!("index_do_end: {index_do_end}");
    }
    println!("sol 2: {sol_2}");
    Ok(())
}
