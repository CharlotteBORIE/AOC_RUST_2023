pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 9 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 9 Part 2");
    println!("{:?}", part2_implementation(input));
}


pub fn bench(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}
pub fn part1_implementation(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let number : Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        //println!("{:?}", number);
        let next = get_next(&number);
        sum += next;
        //println!("next: {}", next);
    }
    sum.to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let number : Vec<i64> = line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        let previous = get_previous(&number);
        sum += previous;
    }
    sum.to_string()
}

fn get_next(number: &Vec<i64>) -> i64 {
    let diff = get_diff_sequence(number);
    if diff.iter().all(|&x| x == 0) {
        return *number.last().unwrap();
    }
    else {
        return get_next(&diff) + *number.last().unwrap();
    }
}

fn get_previous(number: &Vec<i64>) -> i64 {
    let diff = get_diff_sequence(number);
    if diff.iter().all(|&x| x == 0) {
        return *number.first().unwrap();
    }
    else {
        return *number.first().unwrap() - get_previous(&diff) ;
    }
}

fn get_diff_sequence(number: &Vec<i64>) -> Vec<i64> {
    let mut diff = Vec::new();
    for i in 0..number.len()-1 {
        diff.push(number[i+1] - number[i]);
    }
    diff
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 114.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 2.to_string());
    }
}