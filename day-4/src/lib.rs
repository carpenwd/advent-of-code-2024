#![allow(warnings)]
use std::{fs::read_to_string, ops::Sub};

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines()
        .map(String::from)
        .collect()
}

pub fn part_1(puzzle: &Vec<Vec<char>>, length: usize, width: usize) -> u32 {
    let mut total: u32 = 0;
    let diagonals: Vec<(isize, isize)> = [(-1, -1), (-1, 1), (1, 1), (1, -1)].to_vec();
    let cardinals: Vec<(isize, isize)> = [(0, 1), (0, -1), (1, 0), (-1, 0)].to_vec();
    for (x, row) in puzzle.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 'X' {
                for entry in &diagonals {
                    let mut diagonal_vec: Vec<char> = Vec::new();
                    
                    // Need to make sure that we are within the length/width bounds
                    if (x.wrapping_add_signed(entry.0 * 3) >= 0 && x.wrapping_add_signed(entry.0 * 3) < length) && 
                        (y.wrapping_add_signed(entry.1 * 3) >= 0 && y.wrapping_add_signed(entry.1 * 3) < width) {
                            diagonal_vec.push(puzzle[x][y]);
                            diagonal_vec.push(puzzle[x.saturating_add_signed(entry.0)][y.saturating_add_signed(entry.1)]);
                            diagonal_vec.push(puzzle[x.saturating_add_signed(entry.0 * 2)][y.saturating_add_signed(entry.1 * 2)]);
                            diagonal_vec.push(puzzle[x.saturating_add_signed(entry.0 * 3)][y.saturating_add_signed(entry.1 * 3)]);
                    }

                    let diagonal_word: String = diagonal_vec.iter().collect();
                    if diagonal_word == "XMAS" {
                        total += 1;
                    }
                }
                
                for entry in &cardinals {
                    let mut cardinal_vec: Vec<char> = Vec::new();
                    
                    // Need to make sure that we are within the length/width bounds
                    if (x.wrapping_add_signed(entry.0 * 3) >= 0 && x.wrapping_add_signed(entry.0 * 3) < length) && 
                        (y.wrapping_add_signed(entry.1 * 3) >= 0 && y.wrapping_add_signed(entry.1 * 3) < width) {
                            cardinal_vec.push(puzzle[x][y]);
                            cardinal_vec.push(puzzle[x.wrapping_add_signed(entry.0)][y.wrapping_add_signed(entry.1)]);
                            cardinal_vec.push(puzzle[x.wrapping_add_signed(entry.0 * 2)][y.wrapping_add_signed(entry.1 * 2)]);
                            cardinal_vec.push(puzzle[x.wrapping_add_signed(entry.0 * 3)][y.wrapping_add_signed(entry.1 * 3)]);
                    }

                    let cardinal_word: String = cardinal_vec.iter().collect();
                    if cardinal_word == "XMAS" {
                        total += 1;
                    }
                }
            }
        }
    }

    return total;
}

pub fn part_2(puzzle: &Vec<Vec<char>>, length: usize, width: usize) -> u32 {
    let mut total: u32 = 0;
    let diagonals: Vec<(isize, isize)> = [(-1, -1), (-1, 1), (1, 1), (1, -1)].to_vec();
    for (x, row) in puzzle.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == 'A' {
                let mut mas_vec: Vec<String> = Vec::new();
                for entry in &diagonals {
                    let mut diagonal_vec: Vec<char> = Vec::new();
                    // Need to make sure that we are within the length/width bounds
                    if (x.wrapping_add_signed(entry.0) >= 0 && x.wrapping_add_signed(entry.0) < length) &&
                        (x.wrapping_add_signed(entry.0 * -1) >= 0 && x.wrapping_add_signed(entry.0 * -1) < length) &&
                        (y.wrapping_add_signed(entry.1) >= 0 && y.wrapping_add_signed(entry.1) < width) &&
                        (y.wrapping_add_signed(entry.1 * -1) >= 0 && y.wrapping_add_signed(entry.1 * -1) < width) {
                            diagonal_vec.push(puzzle[x.wrapping_add_signed(entry.0)][y.wrapping_add_signed(entry.1)]);
                            diagonal_vec.push(puzzle[x][y]);
                            diagonal_vec.push(puzzle[x.wrapping_add_signed(entry.0 * -1)][y.wrapping_add_signed(entry.1 * -1)]);
                    }

                    let diagonal_word: String = diagonal_vec.iter().collect();
                    if diagonal_word == "MAS" {
                        mas_vec.push(diagonal_word);
                    }
                }
                if mas_vec.len() >= 2 {
                    total += 1;
                }
            }
        }
    }

    return total;
}