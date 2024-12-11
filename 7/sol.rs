use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub struct LineParser<T> {
    reader: BufReader<File>,
    parse_func: Box<dyn Fn(&str) -> T>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> LineParser<T> {
    pub fn new<P: AsRef<Path>>(path: P, parse: Box<dyn Fn(&str) -> T>) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        LineParser {
            reader,
            parse_func: parse,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Iterator for LineParser<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None, // EOF
            Ok(_) => Some((self.parse_func)(line.trim())),
            _ => unreachable!(),
        }
    }
}

fn parse_line(line: &str) -> (i64, Vec<i32>) {
    let mut parts = line.split(": ");
    let result: i64 = parts.next().unwrap().parse().unwrap();
    let oprands: Vec<i32> = parts
        .next()
        .unwrap()
        .split(' ')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    (result, oprands)
}

fn can_reach_exp_result(
    cur_result: &mut i64,
    cur_id: &mut usize,
    operands: &Vec<i32>,
    operators: &mut Vec<char>,
    exp_result: i64,
    operator_set: &Vec<char>,
) -> Option<i64> {
    // have to use all operands
    if *cur_result == exp_result && *cur_id == operands.len() {
        return Some(exp_result);
    }
    // only positive input
    if *cur_id >= operands.len() || *cur_result > exp_result {
        return None;
    }
    for op in operator_set {
        match op {
            '+' => *cur_result += operands[*cur_id] as i64,
            '*' => *cur_result *= operands[*cur_id] as i64,
            '|' => {
                *cur_result = format!("{}{}", cur_result, operands[*cur_id])
                    .parse::<i64>()
                    .unwrap()
            }
            _ => unreachable!(),
        }
        operators.push(*op);
        *cur_id += 1;
        if let Some(result) = can_reach_exp_result(
            cur_result,
            cur_id,
            operands,
            operators,
            exp_result,
            operator_set,
        ) {
            return Some(result);
        }
        // backtrack
        operators.pop();
        *cur_id -= 1;
        match op {
            '+' => *cur_result -= operands[*cur_id] as i64,
            '*' => *cur_result /= operands[*cur_id] as i64,
            '|' => *cur_result /= 10_i64.pow(operands[*cur_id].to_string().len() as u32),
            _ => unreachable!(),
        }
    }
    None
}

fn main() -> io::Result<()> {
    let parser = LineParser::new("7/input.txt", Box::new(parse_line));
    // let sol_1: i64 = parser
    //     .filter_map(|(exp_result, operands)| {
    //         can_reach_exp_result(
    //             &mut (operands[0] as i64),
    //             &mut 1,
    //             &operands,
    //             &mut vec![],
    //             exp_result,
    //             &vec!['+', '*'],
    //         )
    //     })
    //     .sum();
    // println!("sol 1: {sol_1}");
    let sol_2: i64 = parser
        .filter_map(|(exp_result, operands)| {
            can_reach_exp_result(
                &mut (operands[0] as i64),
                &mut 1,
                &operands,
                &mut vec![],
                exp_result,
                &vec!['|', '+', '*'],
            )
        })
        .sum();
    println!("sol 2: {sol_2}");

    Ok(())
}
