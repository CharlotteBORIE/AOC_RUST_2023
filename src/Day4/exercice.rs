pub fn part1() {
    let input = include_str!("Input.txt");
    println!("Day 4 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2() {
    let input = include_str!("Input.txt");
    println!("Day 4 Part 2");
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
    for line in input.lines() {
        let mut counter = 0;
        let mut line_split = line.split(['|', ':']).collect::<Vec<&str>>();
        let wining = line_split[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let has = line_split[2]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        for elem in has {
            if wining.contains(&elem) {
                if counter == 0 {
                    counter = 1;
                } else {
                    counter *= 2;
                }
            }
        }

        sum += counter;
    }
    sum.to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let mut sum = vec![1; input.lines().count()];
    for line in input.lines() {
        let mut line_split = line.split(['|', ':']).collect::<Vec<&str>>();
        let index = line_split[0].strip_prefix("Card").unwrap().split_whitespace().collect::<Vec<&str>>()[0].parse::<usize>().unwrap() -1;
        let wining = line_split[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let has = line_split[2]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut index_to_add = 0;
        for elem in has {
            if wining.contains(&elem) {
                if index + index_to_add < sum.len() {
                    index_to_add += 1;
                    sum[index + index_to_add] += sum[index];
                }
            }
        }
    }
    sum.iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 13.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 30.to_string());
    }
}
