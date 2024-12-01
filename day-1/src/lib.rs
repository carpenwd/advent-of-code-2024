use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn find_distance(left_vec: &Vec<i32>, right_vec: &Vec<i32>) -> i32 {
    let mut difference: i32 = 0;
    if left_vec.len() == right_vec.len() {
        for i in 0..left_vec.len() {
            difference += (left_vec[i] - right_vec[i]).abs();
        }
    }

    return difference;
}

pub fn find_similarity(left_vec: &Vec<i32>, right_vec: &Vec<i32>) -> i32 {
    let mut similarity: i32 = 0;
    if left_vec.len() == right_vec.len() {
        for left_entry in left_vec {
            let mut count = 0;
            for right_entry in right_vec {
                if left_entry == right_entry {
                    count += 1;
                }
            }

            similarity += left_entry * count;
        }
    }

    return similarity;
}