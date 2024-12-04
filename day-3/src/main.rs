use day_3::read_lines;
use regex::Regex;

fn main() {
    let lines = read_lines("day-3/input/input.txt");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    // For each line in the input
    // Create a regex string that matches `mul\((\d{1,3}),(\d{1,3})\)`
    // For each one that matches, then multiply the first group by the second
}
