use day_3::{part_1, part_2, read_lines};

fn main() {
    let lines = read_lines("day-3/input/input.txt");
    println!("Part-1 Total: {}", part_1(&lines));
    println!("Part-2 Total: {}", part_2(&lines));
}
