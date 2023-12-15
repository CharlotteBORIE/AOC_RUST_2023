use std::iter::Rev;
use std::slice::Iter;

pub fn part1() {
    let input = include_str!("Input.txt");
    println!("Day 12 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2() {
    let input = include_str!("Input.txt");
    println!("Day 12 Part 2");
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
    let mut sum = 0;
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<&str>>();
        let mut map = split[0].to_string();
        map.push('.');
        let map = map.as_str();

        let repartition = split[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let arrangement = get_arrangement(map, &repartition,0,0);
        sum += arrangement;
    }
    sum.to_string()
}
pub fn part2_implementation(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<&str>>();

        let mut new_map = vec![split[0]; 5].join("?");
        new_map.push('.');
        let new_repartition = vec![split[1]; 5].join(",");

        let map = new_map.as_str();
        let repartition = new_repartition
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let arrangement = get_arrangement(map, &repartition,0,0);
         sum += arrangement;
    }
    sum.to_string()
}
fn get_arrangement(mapping: &str, repartition: &Vec<usize>, index_map : usize, index_rep : usize) -> usize {

    if repartition.len() == index_rep {
        return !mapping[index_map..].chars().any(|x| x == '#') as usize;
    }
    
    let value = repartition[index_rep];
    if mapping.len() - index_map < value + 1{
        //we can't do the repartition
        return 0;
    }

    let mut sum = 0;
    if mapping.chars().nth(index_map).unwrap() != '#' {
        sum+=get_arrangement(mapping, repartition, index_map + 1, index_rep);
    }
    if !mapping[index_map..index_map+value].chars().any(|x| x == '.')
        && mapping.chars().nth(index_map + value).unwrap() != '#'
    {
        sum+=get_arrangement(mapping, repartition, index_map + value + 1, index_rep + 1);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 21.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 525152.to_string());
    }
}
