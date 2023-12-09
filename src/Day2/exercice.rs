use std::cmp::max;

pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 2 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day2 Part 2");
    println!("{:?}", part2_implementation(input));
}

pub fn bench(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}

pub fn part1_implementation(input: &str) -> String {
    let mut sum = 0;
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;
    for line in input.lines() {
        let mut line_split = line.split([';',':']).collect::<Vec<&str>>();
        let index = line_split[0].strip_prefix("Game ").unwrap().parse::<usize>().unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        line_split.remove(0);

        let mut possible = true;
        for &game in line_split.iter() {
            let game_split = game.split(',').collect::<Vec<&str>>();
            for &elem in game_split.iter() {
                let split = elem.split_whitespace().collect::<Vec<&str>>();
                if split[1] == "red" {
                    if red_limit < split[0].parse::<usize>().unwrap(){
                        possible=false;
                        break;
                    }
                }
                if split[1] == "green" {
                    if green_limit < split[0].parse::<usize>().unwrap(){
                        possible=false;
                        break;
                    }
                }
                if split[1] == "blue" {
                    if blue_limit < split[0].parse::<usize>().unwrap(){
                        possible=false;
                        break;
                    }
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            sum += index;
        }

    }
    sum.to_string()
}

pub fn part2_implementation(input: &str) -> String  {
        let mut sum = 0;
        let red_limit = 12;
        let green_limit = 13;
        let blue_limit = 14;
        for line in input.lines() {
            let mut line_split = line.split([';',':']).collect::<Vec<&str>>();
            let index = line_split[0].strip_prefix("Game ").unwrap().parse::<usize>().unwrap();
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            line_split.remove(0);

            for &game in line_split.iter() {
                let game_split = game.split(',').collect::<Vec<&str>>();
                for &elem in game_split.iter() {
                    let split = elem.split_whitespace().collect::<Vec<&str>>();
                    if split[1] == "red" {
                        red = max(red, split[0].parse::<usize>().unwrap());
                    }
                    if split[1] == "green" {
                        green = max(green, split[0].parse::<usize>().unwrap());
                    }
                    if split[1] == "blue" {
                        blue = max(blue, split[0].parse::<usize>().unwrap());
                    }
                }
            }
            sum += red*blue*green
        }
        sum.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 8.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 2286.to_string());
    }
}