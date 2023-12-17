use std::collections::HashMap;
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
    let mut arrangement:HashMap<(usize,usize),usize> = HashMap::new();
    set_cell(&mut arrangement,0,0,1);
    for (index,char) in mapping.chars().enumerate() {
        let mut new_arrangement = HashMap::new();
        let possible_chars = match char
        {
            '.' => vec!['.'],
            '#' => vec!['#'],
            '?' => vec!['.', '#'],
            _ => panic!("Unknown char")
        };
        for &(so_far,index) in arrangement.keys(){
            let value = arrangement[&(so_far,index)];
            for possible_char in possible_chars.iter(){
                if index == repartition.len(){
                    //we are at the end of the repartition
                    if possible_char == &'.'{
                        add_cell(&mut new_arrangement,so_far,index,value);
                    }
                }
                else{
                    if so_far == repartition[index]{
                        //we can't do the repartition
                        if possible_char == &'.'{
                            add_cell(&mut new_arrangement,0,index+1,value);
                        }
                    }
                    else{
                        if possible_char == &'.' && so_far ==0{
                            add_cell(&mut new_arrangement,so_far,index,value);
                        }
                        if possible_char == &'#'{
                            add_cell(&mut new_arrangement,so_far +1,index,value);
                        }
                    }
                }
            }
        }
        arrangement = new_arrangement;
    }

    let first = arrangement.get(&(0,repartition.len()));
    let last = arrangement.get(&(*repartition.last().unwrap(),repartition.len()));

    match (first,last){
        (Some(&x),Some(&y)) => x+y,
        (Some(&x),None) => x,
        (None,Some(&y)) => y,
        (None,None) => 0
    }
}

fn add_cell(hash_map: &mut HashMap<(usize, usize), usize>, key1: usize, key2: usize, value: usize) {
    let cell = hash_map.entry((key1, key2)).or_insert(0);
    *cell += value;
}

fn set_cell(hash_map: &mut HashMap<(usize, usize), usize>, key1: usize, key2: usize, value: usize) {
    let cell = hash_map.entry((key1, key2)).or_insert(0);
    *cell = value;
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
