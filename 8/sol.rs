use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use itertools::Itertools;

fn read_map(path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(path).unwrap();
    file.lines()
        .map(|line| line.chars().collect())
        .collect_vec()
}

fn print_map(map: &Vec<Vec<char>>) {
    map.iter()
        .for_each(|line| println!("{}", line.iter().collect::<String>()));
    println!();
}

fn record_locations(map: &Vec<Vec<char>>) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut result: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    map.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, ch)| {
            if *ch != '.' {
                result
                    .entry(*ch)
                    .and_modify(|e| {
                        e.insert((i, j));
                    })
                    .or_insert(HashSet::from([(i, j)]));
            }
        })
    });
    result
}

fn record_all_targets(map: &mut Vec<Vec<char>>) {
    let locations = record_locations(map);
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    locations.values().for_each(|v| {
        v.iter().combinations(2).for_each(|p| {
            let (pos_1, pos_2) = (p[0], p[1]);
            let target_1 = (
                2 * pos_1.0 as i32 - pos_2.0 as i32,
                2 * pos_1.1 as i32 - pos_2.1 as i32,
            );
            let target_2 = (
                2 * pos_2.0 as i32 - pos_1.0 as i32,
                2 * pos_2.1 as i32 - pos_1.1 as i32,
            );
            if (0..rows).contains(&target_1.0) && (0..cols).contains(&target_1.1) {
                map[target_1.0 as usize][target_1.1 as usize] = '~';
            }
            if (0..rows).contains(&target_2.0) && (0..cols).contains(&target_2.1) {
                map[target_2.0 as usize][target_2.1 as usize] = '~';
            }
        })
    })
}

fn record_all_in_line_targets(map: &mut Vec<Vec<char>>) {
    let locations = record_locations(map);
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    locations.values().for_each(|v| {
        let mut i = 1;
        loop {
            let mut within_map = false;
            v.iter().combinations(2).for_each(|p| {
                let (pos_1, pos_2) = (p[0], p[1]);
                let target_1 = (
                    pos_1.0 as i32 + i * (pos_1.0 as i32 - pos_2.0 as i32),
                    pos_1.1 as i32 + i * (pos_1.1 as i32 - pos_2.1 as i32),
                );
                let target_2 = (
                    pos_2.0 as i32 + i * (pos_2.0 as i32 - pos_1.0 as i32),
                    pos_2.1 as i32 + i * (pos_2.1 as i32 - pos_1.1 as i32),
                );
                if (0..rows).contains(&target_1.0) && (0..cols).contains(&target_1.1) {
                    map[target_1.0 as usize][target_1.1 as usize] = '~';
                    map[pos_1.0][pos_1.1] = '~';
                    map[pos_2.0][pos_2.1] = '~';
                    within_map = true;
                }
                if (0..rows).contains(&target_2.0) && (0..cols).contains(&target_2.1) {
                    map[target_2.0 as usize][target_2.1 as usize] = '~';
                    map[pos_1.0][pos_1.1] = '~';
                    map[pos_2.0][pos_2.1] = '~';
                    within_map = true;
                }
            });
            if !within_map {
                break;
            }
            print_map(map);
            i += 1;
        }
    })
}

fn main() -> io::Result<()> {
    let mut map = read_map("8/sample.txt");
    // record_all_targets(&mut map);
    // print_map(&map);
    // let sol_1 = map.iter().flatten().filter(|&ch| *ch == '~').count();
    // println!("sol 1: {sol_1}");
    record_all_in_line_targets(&mut map);
    print_map(&map);
    let sol_2 = map.iter().flatten().filter(|&ch| *ch == '~').count();
    println!("sol 2: {sol_2}");
    Ok(())
}
