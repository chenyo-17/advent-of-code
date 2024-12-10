use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

use itertools::Itertools;

fn read_map(file: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let content = fs::read_to_string(file).unwrap();
    let mut pos_start = None;
    let map: Vec<Vec<char>> = content
        .lines()
        .enumerate()
        .map(|(row, line)| {
            if pos_start.is_none() {
                let mut result = vec![];
                for (col, ch) in line.chars().enumerate() {
                    if ch == '^' {
                        pos_start = Some((row, col));
                    }
                    result.push(ch);
                }
            }
            line.chars().collect_vec()
        })
        .collect();
    (map, pos_start.unwrap())
}

fn print_map(map: &Vec<Vec<char>>) {
    map.iter()
        .for_each(|line| println!("{}", line.iter().collect::<String>()));
    println!();
}

fn simulate_walk_or_find_loop(map: &mut Vec<Vec<char>>, start: (usize, usize)) -> bool {
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    let (mut cur_r, mut cur_c) = start;
    let mut next_move = (-1, 0);
    let mut additional_pos_directions: HashMap<(usize, usize), HashSet<char>> = HashMap::new(); // store additional directions a position has seen
    loop {
        let direction = match next_move {
            (-1, 0) => '^',
            (0, 1) => '>',
            (1, 0) => 'v',
            (0, -1) => '<',
            _ => unreachable!(),
        };
        // check loop, need to skip the start as it already has a ^
        if (cur_r, cur_c) != start && map[cur_r][cur_c] == direction
            || additional_pos_directions
                .get(&(cur_r, cur_c))
                .unwrap_or(&HashSet::new())
                .contains(&direction)
        {
            return true;
        }
        if map[cur_r][cur_c] != '.' {
            additional_pos_directions
                .entry((cur_r, cur_c))
                .and_modify(|e| {
                    e.insert(direction);
                })
                .or_insert(HashSet::from([direction]));
        } else {
            map[cur_r as usize][cur_c as usize] = direction;
        }
        let (next_r, next_c) = (cur_r as i32 + next_move.0, cur_c as i32 + next_move.1);
        if !(0..rows).contains(&next_r) || !(0..cols).contains(&next_c) {
            break;
        }
        if map[next_r as usize][next_c as usize] == '#' {
            next_move = match next_move {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };
        } else {
            (cur_r, cur_c) = (next_r as usize, next_c as usize);
        }
    }
    false
}

fn new_obstacle_causes_loop(
    obstacle_pos: (usize, usize),
    map: &Vec<Vec<char>>,
    start: (usize, usize),
) -> bool {
    let mut map_clone = map.clone();
    map_clone[obstacle_pos.0][obstacle_pos.1] = '#';
    simulate_walk_or_find_loop(&mut map_clone, start)
}

fn main() -> io::Result<()> {
    let (mut map, start) = read_map("6/input.txt");
    let original_map = map.clone();
    simulate_walk_or_find_loop(&mut map, start);
    // print_map(&map);
    let path_signs = HashSet::from(['<', 'v', '>', '^']);
    let paths: Vec<(usize, usize)> = (0..map.len())
        .flat_map(|row| (0..map[row].len()).map(move |col| (row, col)))
        .filter(|&(row, col)| path_signs.contains(&map[row][col]))
        .collect();
    let sol_1 = paths.len();
    println!("sol 1: {sol_1}");

    let sol_2 = paths
        .iter()
        .filter(|(r, c)| (*r, *c) != start)
        .filter(|(r, c)| new_obstacle_causes_loop((*r, *c), &original_map, start))
        .count();
    println!("sol 2: {sol_2}");

    Ok(())
}
