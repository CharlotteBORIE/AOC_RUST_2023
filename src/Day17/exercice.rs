use grid::Grid;
use std::cmp::min;

pub fn part1() {
    let input = include_str!("Input.txt");
    println!("Day 17 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2() {
    let input = include_str!("Input.txt");
    println!("Day 17 Part 2");
    println!("{:?}", part2_implementation(input));
}

pub fn bench() {
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}
pub fn bench1() {
    let input = include_str!("Input.txt");
    part1_implementation(input);
}
pub fn bench2() {
    let input = include_str!("Input.txt");
    part2_implementation(input);
}

pub fn part1_implementation(input: &str) -> String {
    let cols = input.lines().next().unwrap().len();
    let mut grid = Grid::from_vec(
        input
            .chars()
            .filter(|&c| c != '\n')
            .map(|x| x.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
        cols,
    );
    let already_visited = vec![];

    if let Some(path) = get_paths(&grid, 0, 0, Direction::Right, 0, &already_visited) {
   (count_value(&grid, &path) - grid.get(0, 0).unwrap())
    .to_string()
    } else {
        0.to_string()
    }
}

fn get_paths(
    grid: &Grid<usize>,
    row: usize,
    column: usize,
    direction: Direction,
    count: usize,
    already_visited: & Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize, Direction, usize)>> {

    //println!("{} {} {:?} {}", row, column, direction, count);
    let &cell = grid.get(row, column).unwrap();
    let mut path = vec![(row, column, direction, count)];
    if row == grid.rows() - 1 && column == grid.cols() - 1 {
        return Some(path);
    }
    let mut already_visited = already_visited.clone();
    already_visited.push((row, column));

    let mut paths = vec![];

    if column + 1 < grid.cols() && direction!=Direction::Left && !(direction == Direction::Right && count >= 3) && !already_visited.iter().any(|x| x.0 == row && x.1 == column + 1) {
        let mut new_count = count;
        if direction != Direction::Right {
            new_count = 0;
        }
        let mut new_path = path.clone();
        if let Some(mut elem) = get_paths(grid, row , column + 1, Direction::Right, new_count + 1, &already_visited){
            new_path.append(&mut elem);
            paths.push(new_path);
        }
    }
    if row + 1 < grid.rows() && direction!=Direction::Up && !(direction == Direction::Down && count>=3) && !already_visited.iter().any(|x| x.0 == row + 1 && x.1 == column) {
        let mut new_count = count;
        if direction != Direction::Down {
            new_count = 0;
        }
        let mut new_path = path.clone();
        if let Some(mut elem) = get_paths(grid, row+ 1, column , Direction::Down, new_count + 1, &already_visited){
            new_path.append(&mut elem);
            paths.push(new_path);
        }
    }
    if column > 0 && direction!=Direction::Right && !(direction == Direction::Left && count>=3) && !already_visited.iter().any(|x| x.0 == row && x.1 == column - 1){
        let mut new_count = count;
        if direction != Direction::Left {
            new_count = 0;
        }
        let mut new_path = path.clone();
        if let Some(mut elem) = get_paths(grid, row , column - 1, Direction::Left, new_count + 1, &already_visited){
            new_path.append(&mut elem);
            paths.push(new_path);
        }
    }
    if row > 0 && direction!=Direction::Down && !(direction == Direction::Up && count>=3) && !already_visited.iter().any(|x| x.0 == row - 1 && x.1 == column) {
        let mut new_count = count;
        if direction != Direction::Up {
            new_count = 0;
        }
        let mut new_path = path.clone();
        if let Some(mut elem) = get_paths(grid, row- 1, column, Direction::Up, new_count + 1, &already_visited){
            new_path.append(&mut elem);
            paths.push(new_path);
        }

    }

    if paths.len() == 0 {
        return None;
    }

    Some(get_min(grid, paths))
}

fn get_min(
    grid: &Grid<usize>,
    paths: Vec<Vec<(usize, usize, Direction, usize)>>,
) -> Vec<(usize, usize, Direction, usize)> {
    let path_values = paths
        .iter()
        .enumerate()
        .map(|(index, path)| (index, count_value(grid, path)))
        .collect::<Vec<(usize, usize)>>();
    paths[path_values
        .iter()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0]
        .clone()
}

fn count_value(grid: &Grid<usize>, path: &Vec<(usize, usize, Direction, usize)>) -> usize {
    path.iter()
        .fold(0, |acc, x| acc + grid.get(x.0, x.1).unwrap())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part2_implementation(input: &str) -> String {
    let mut product = 0;
    for line in input.lines() {
        let number = line.parse::<i32>().unwrap();
        product *= number;
    }
    product.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 102.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 0.to_string());
    }
}
