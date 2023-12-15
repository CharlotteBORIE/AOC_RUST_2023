use itertools::Itertools;

pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 15 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 15 Part 2");
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
    let mut sum = 0;
    let lines = input.split(",").collect::<Vec<&str>>();
    for line in lines {
        sum += get_hash_number(line);
    }
    sum.to_string()
}

fn get_hash_number(line: &str) -> usize {
    let mut number = 0;
    for char in line.chars() {
        number += char as usize;
        number *= 17;
        number %= 256;
    }
    number
}

pub fn part2_implementation(input: &str) -> String {
    let mut sum = 0;
    let mut boxes:Vec<Vec<(&str,usize)>> = vec![vec![]; 256];
    let lines = input.split(",").collect::<Vec<&str>>();
    for line in lines {
        if line.chars().last().unwrap() == '-' {
            let label = line.strip_suffix("-").unwrap();
            let box_number = get_hash_number(label);
            boxes[box_number as usize].retain(|(x,_num)| *x != label && *x != "");

        }
        else{
            let split = line.split("=").collect::<Vec<&str>>();
            let label = split[0];
            let box_number = get_hash_number(label);
            let value = split[1].parse::<usize>().unwrap();
            let index = boxes[box_number as usize].iter().position(|(x,_num)| *x == label);
            if let Some(place) = index{
                boxes[box_number][place].1 = value;
            }
            else{
                boxes[box_number].push((label,value));
            }
        };


    }
    for (index_box,boxe) in boxes.iter().enumerate(){
        for (index_lens, (_label,value)) in boxe.iter().enumerate(){
            sum += (index_box+1) * (index_lens+1) * value;
        }
    }
    sum.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 1320.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 145.to_string());
    }
}