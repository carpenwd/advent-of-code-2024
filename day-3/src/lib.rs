use std::fs::read_to_string;
use regex::Regex;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn part_1(input_lines: &Vec<String>) -> i32 {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
        let mut multiply_total: i32 = 0;
        for line in input_lines {
            let line_matches: Vec<(i32, i32)> = re.captures_iter(&line).map(|line_match| {
                let (_, [m1, m2]) = line_match.extract();
                (m1.parse::<i32>().unwrap(), m2.parse::<i32>().unwrap())
            }).collect();
    
            for entry in line_matches {
                multiply_total += entry.0 * entry.1;
            }
        }
    
        return multiply_total;
}

pub fn part_2(input_lines: &Vec<String>) -> i32 {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    let mut multiply_vals = true;
    let mut multiply_total: i32 = 0;
    for line in input_lines {
        let line_matches: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();

        for entry in line_matches {
            if entry == "do()" {
                multiply_vals = true;
            } else if entry == "don't()" {
                multiply_vals = false;
            } else {
                if multiply_vals {
                    let clean_entry = entry.replace("mul(", "").replace(")", "");
                    let digits: Vec<&str> = clean_entry.split(",").collect();
                    multiply_total += digits[0].parse::<i32>().unwrap() * digits[1].parse::<i32>().unwrap();
                }
            }
        }
    }

    return multiply_total;
}