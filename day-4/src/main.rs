use day_4::{part_1, read_lines};

fn main() {
    let lines = read_lines("day-4/input/input.txt");
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars = line.chars().collect();
        puzzle.push(chars);
    }

    let length: usize = puzzle.len();
    let width: usize = puzzle[0].len();
    println!("Part-1 Total: {}", part_1(&puzzle, length, width));
    //println!("Part-2 Total: {}", part_1(&puzzle, length, width));
}
