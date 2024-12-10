use itertools::Itertools;
use std::fs;

fn read_matrix(file: &str) -> Vec<Vec<char>> {
    let text = fs::read_to_string(file).unwrap();
    text.lines().map(|line| line.chars().collect()).collect()
}

fn number_of_xmas_at_x(matrix: &Vec<Vec<char>>, row_x: usize, col_x: usize) -> usize {
    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),   // right
        (0, -1),  // left
        (-1, 0),  // down
        (1, 0),   // up
        (1, -1),  // down left
        (1, 1),   // down right
        (-1, -1), // up left
        (-1, 1),  // up right
    ];
    let all_diffs = DIRECTIONS.iter().map(|&(dr, dc)| {
        (1..=3)
            .map(|i| (dr * i, dc * i))
            .collect::<Vec<(i32, i32)>>()
    });
    let find_mas_in_one_diff = |diff: &[(i32, i32)]| -> bool {
        let pattern = ['M', 'A', 'S'];
        let (rows, cols) = (matrix.len() as i32, matrix[0].len() as i32);
        let (r, c) = (row_x as i32, col_x as i32);
        diff.iter().enumerate().all(|(id, &(dr, dc))| {
            let (r, c) = (dr + r, dc + c);
            (0..rows).contains(&r)
                && (0..cols).contains(&c)
                && pattern[id] == matrix[r as usize][c as usize]
        })
    };
    all_diffs.filter(|diff| find_mas_in_one_diff(diff)).count()
}

fn xmas_at_a(matrix: &[Vec<char>], row_a: usize, col_a: usize) -> bool {
    let left_diagonal = ((-1, -1), (1, 1));
    let right_diagonal = ((1, -1), (-1, 1));
    let pattern = ['M', 'S'];
    let find_mas_in_one_diagonal = |diagonal: &((i32, i32), (i32, i32))| -> bool {
        let (rows, cols) = (matrix.len() as i32, matrix[0].len() as i32);
        let (r, c) = (row_a as i32, col_a as i32);
        let (df, ds) = (diagonal.0, diagonal.1);
        let (rf, cf) = (df.0 + r, df.1 + c);
        let (rs, cs) = (ds.0 + r, ds.1 + c);
        if !((0..rows).contains(&rf)
            && (0..cols).contains(&cf)
            && (0..rows).contains(&rs)
            && (0..cols).contains(&cs))
        {
            return false;
        }
        let (first_char, second_char) = (
            matrix[rf as usize][cf as usize],
            matrix[rs as usize][cs as usize],
        );
        pattern.contains(&first_char) && pattern.contains(&second_char) && first_char != second_char
    };
    find_mas_in_one_diagonal(&left_diagonal) && find_mas_in_one_diagonal(&right_diagonal)
}

fn main() -> std::io::Result<()> {
    let file = "4/input.txt";
    let matrix = read_matrix(file);
    // let sol_1: usize = matrix
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(i, row)| row.iter().positions(|&c| c == 'X').map(move |j| (i, j)))
    //     .map(|(row_x, col_x)| number_of_xmas_at_x(&matrix, row_x, col_x))
    //     .sum();
    // println!("sol 1: {sol_1}");
    let sol_2: usize = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().positions(|&c| c == 'A').map(move |j| (i, j)))
        .filter(|&(row_a, col_a)| xmas_at_a(&matrix, row_a, col_a))
        .count();
    println!("sol 2: {sol_2}");
    Ok(())
}
