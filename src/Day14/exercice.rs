use std::iter::StepBy;
use std::panic::panic_any;
use std::slice::Iter;
use grid::Grid;

pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 14 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 14 Part 2");
    println!("{:?}", part2_implementation(input));
}

pub fn bench(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}
pub fn bench1(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
}
pub fn bench2(){
    let input = include_str!("Input.txt");
    part2_implementation(input);
}

pub fn part1_implementation(input: &str) -> String {
    let cols = input.lines().next().unwrap().len();
    let mut grid: Grid<char> = Grid::from_vec(input.chars().filter(|&c| c != '\n').collect(), cols);
    let mut sum = 0;
    for column in grid.iter_cols() {
        let number = tilt_north(column);
        sum += number;
    }
    sum.to_string()
}

fn tilt_north(column: StepBy<Iter<char>>) -> usize {
    let mut sum = 0;
    let mut count = 0;
    let mut block_index = column.len();
    let mut current_index = column.len();
    for &row in column{
        current_index -= 1;
        match row {
            '.' => {},
            '#' => {
                //println!("count: {} {}", block_index, count);
                sum += (block_index-count+1..=block_index).sum::<usize>();
                block_index = current_index;
                count = 0 ;
            }
            'O' => {
                count+= 1;
            }
            _ => panic!("Invalid char"),
        }

        //println!("sum: {}", sum);
    }
    sum += (block_index-count+1..=block_index).sum::<usize>();
    sum
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
        assert_eq!(part1_implementation(input), 136.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 0.to_string());
    }
}