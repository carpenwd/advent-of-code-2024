use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn check_level(report_line: &Vec<i32>) -> bool {
    let mut diff_vec: Vec<i32> = Vec::new();
    for i in 0..(report_line.len() - 1) {
        let left_side: i32 = report_line[i];
        let right_side: i32 = report_line[i + 1];

        diff_vec.push(left_side - right_side);
    }

    let all_increasing: bool = diff_vec.iter().all(|&entry| entry < 0);
    let all_decreasing: bool = diff_vec.iter().all(|&entry| entry > 0);

    return all_increasing || all_decreasing;
}

pub fn check_adjacency(report_line: &Vec<i32>) -> bool {
    let mut result: bool = true;
    for i in 0..(report_line.len() - 1) {
        let left_side: i32 = report_line[i];
        let right_side: i32 = report_line[i + 1];

        let difference = (left_side - right_side).abs();
        if 1 <= difference && difference <= 3 {
            result &= true;
        } else {
            result &= false;
        }
    }

    return result;
}