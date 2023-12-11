use itertools::process_results;
use std::collections::HashMap;

pub fn part1() {
    let input = include_str!("Input.txt");
    println!("Day 8 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2() {
    let input = include_str!("Input.txt");
    println!("Day 8 Part 2");
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
    let mut count = 0;
    let instruction = get_instruction(input);
    let dictionary = get_dictionary(input);

    get_path_length_from_start(&instruction, &dictionary, "AAA", |current| {
        get_end(current, true)
    })
    .to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let mut count = 0;
    let instruction = get_instruction(input);
    let dictionary = get_dictionary(input);
    let list_currents = dictionary
        .keys()
        .clone()
        .filter(|x| x.chars().last() == Some('A'))
        .collect::<Vec<&&str>>();
    let counts = list_currents
        .iter()
        .map(|x| {
            get_path_length_from_start(&instruction, &dictionary, x, |current| {
                get_end(current, false)
            })
        })
        .collect::<Vec<u64>>();
    counts
        .iter()
        .fold(counts[0], |acc, &x| get_lcm(acc, x))
        .to_string()
}

fn get_instruction(input: &str) -> Vec<Instruction> {
    let instruction = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<Instruction>>();
    instruction
}
fn get_dictionary(input: &str) -> HashMap<&str, (&str, &str)> {
    let mut dictionnary: HashMap<&str, (&str, &str)> = HashMap::with_capacity(1024);
    for (index, line) in input.lines().enumerate() {
        if index <= 1 {
            continue;
        }
        let split = line
            .split([' ', '(', ',', ')', '='])
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();
        dictionnary.insert(split[0], (split[1], split[2]));
    }
    dictionnary
}

fn get_path_length_from_start(
    instruction: &Vec<Instruction>,
    dictionary: &HashMap<&str, (&str, &str)>,
    start: &str,
    end: fn(&str) -> bool,
) -> u64 {
    let mut count = 0;
    let mut current = start;
    let mut instruction_pattern = instruction.iter();
    while end(&current) {
        let (left, right) = dictionary.get(current).unwrap();
        current = match instruction_pattern.next() {
            Some(Instruction::Left) => left,
            Some(Instruction::Right) => right,
            _ => {
                instruction_pattern = instruction.iter();
                continue;
            }
        };
        count += 1;
    }
    count
}

fn get_end(current: &str, part1: bool) -> bool {
    if part1 {
        return current != "ZZZ";
    }
    current.chars().last() != Some('Z')
}

fn get_lcm(a: u64, b: u64) -> u64 {
    a * b / get_gcd(a, b)
}

fn get_gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub enum Instruction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 2.to_string());

        let input = include_str!("Example2.txt");
        assert_eq!(part1_implementation(input), 6.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example3.txt");
        assert_eq!(part2_implementation(input), 6.to_string());
    }
}
