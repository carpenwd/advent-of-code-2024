use::day_2::{check_adjacency, check_level, read_lines};

fn main() {
    let lines = read_lines("day-2/input/input.txt");
    let mut p1_safe_reports = 0;
    let mut p2_safe_reports = 0;
    for line in lines {
        let report_line: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();

        // Check to make sure each line in the report is all increasing or all decreasing
        let mut level_check = check_level(&report_line);

        // Check to make sure any two adjacent levels differ by at least one and at most three
        let mut adjancent_check = check_adjacency(&report_line);

        if level_check && adjancent_check {
            p1_safe_reports += 1;
            p2_safe_reports += 1;
        } else {
            // Now we need to see if any of the unsafe lines can become safe by removing a single element
            for i in 0..report_line.len() {
                let mut vec: Vec<i32> = report_line.to_vec();
                vec.remove(i);

                level_check = check_level(&vec);
                adjancent_check = check_adjacency(&vec);
                if level_check && adjancent_check {
                    p2_safe_reports += 1;
                    continue;
                }
            }
        }
    }

    println!("Number of safe reports (part 1): {}", p1_safe_reports);
    println!("Number of safe reports (part 2): {}", p2_safe_reports);
}