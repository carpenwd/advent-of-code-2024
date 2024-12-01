use day_1::{find_distance, find_similarity, read_lines};

fn main() {
    let lines = read_lines("day-1/input/input.txt");
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();

    println!("Length of input: {}", lines.len());
    
    for line in lines {
        let temp_vec: Vec<&str> = line.split("   ").collect();
        left_side.push(temp_vec[0].parse().unwrap());
        right_side.push(temp_vec[1].parse().unwrap());
        // Do the maths
    }

    // Sanity checking
    println!("Length of left input: {}", left_side.len());
    println!("Length of left input: {}", right_side.len());

    // Sorting the vectors
    left_side.sort();
    right_side.sort();

    // Find the distance between each left side / right side pair
    println!("Distance difference between left side and right side {}", find_distance(&left_side, &right_side));

    // Find the similarity score between each left side / right side pair
    println!("Similarity score between left side and right side {}", find_similarity(&left_side, &right_side));
}