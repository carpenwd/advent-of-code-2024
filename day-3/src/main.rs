use day_3::read_lines;
use regex::Regex;

fn main() {
    let lines = read_lines("day-3/input/input.txt");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    // For each line in the input
    // Create a regex string that matches `mul\((\d{1,3}),(\d{1,3})\)`
    // For each one that matches, then multiply the first group by the second
    let mut multiply_total: i32 = 0;
    for line in lines {
        let line_matches: Vec<(i32, i32)> = re.captures_iter(&line).map(|line_match| {
            let (_, [m1, m2]) = line_match.extract();
            (m1.parse::<i32>().unwrap(), m2.parse::<i32>().unwrap())
        }).collect();
        //println!("{:?}", line_matches);

        for entry in line_matches {
            //println!("{:?}", entry);
            multiply_total += entry.0 * entry.1;
        }
    }

    println!("{}", multiply_total);
}
