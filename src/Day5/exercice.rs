use std::cmp::Ordering::{Equal, Greater, Less};

pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 5 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 5 Part 2");
    println!("{:?}", part2_implementation(input));
}

pub fn bench(){
    let input = include_str!("Input.txt");
    part1_implementation(input);
    part2_implementation(input);
}

pub fn part1_implementation(input: &str) -> String {
    let first_line: Vec<&str> = input.lines().next().unwrap().strip_prefix("seeds: ").unwrap().split(" ").collect();
    let mut seeds: Vec<u64> = first_line.iter().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut seeds_changed = vec![false; seeds.len()];
    for line in input.lines() {
        if line.contains("map:"){
            seeds_changed = vec![false; seeds.len()];
        }
        else if line.is_empty() || line.chars().next().is_some() && line.chars().next().unwrap().is_alphabetic() { continue;  }
        else
        {
            let params = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            for i in 0..seeds.len(){
                let mut seed = seeds[i];
                if !seeds_changed[i] && seed >= params[1] && seed - params[1] < params[2] {
                    seed = params[0] + seed - params[1] ;
                    seeds[i] = seed;
                    seeds_changed[i] = true;
                }
            }

        }
    }
    seeds.sort();
    seeds.first().unwrap().to_string()
}

pub fn part2_implementation(input: &str) -> String {
    let first_line: Vec<&str> = input.lines().next().unwrap().strip_prefix("seeds: ").unwrap().split(" ").collect();
    let seeds_range: Vec<u64> = first_line.iter().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut seeds: Vec<(u64,u64,bool)> = vec![];
    for i in 0..seeds_range.len()/2{
        let first = seeds_range[i*2];
        let last = first + seeds_range[i*2+1];
        seeds.push((first,last,false));
    }
    for line in input.lines() {
        if line.contains("map:"){
            for elem in seeds.iter_mut(){
                elem.2 = false;
            }
        }
        else if line.is_empty() || line.chars().next().is_some() && line.chars().next().unwrap().is_alphabetic() { continue;  }
        else
        {
            let params = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            let bottom = params[1];
            let top = bottom + params[2];
            let range = params[2];
            let new_first = params[0];
            for i in 0..seeds.len(){
                if seeds[i].2 { continue; }

                let bottom_seed = seeds[i].0;
                let top_seed = seeds[i].1;

                if top_seed <= bottom || bottom_seed >= top { continue;  }

                match (bottom_seed.cmp(&bottom),top_seed.cmp(&top)) {
                    (Less,Less) |(Less,Equal)=> {
                        let new_bottom_seed = new_first;
                        let new_top_seed = new_first + top_seed - bottom;
                        seeds.push((new_bottom_seed,new_top_seed,true));

                        seeds[i].1 = bottom;
                    }
                    (Equal,Less) | (Equal,Equal) => {
                        seeds[i].0 = new_first;
                        seeds[i].1 = new_first + range;
                        seeds[i].2 = true;
                    }
                    (Greater,Less) | (Greater,Equal) => {
                        seeds[i].0 = new_first + bottom_seed - bottom;
                        seeds[i].1 = new_first + top_seed - bottom;
                        seeds[i].2 = true;
                    }
                    (Less,Greater) => {
                        let new_bottom_seed = new_first;
                        let new_top_seed = new_first + range;
                        seeds.push((new_bottom_seed,new_top_seed,true));

                        seeds[i].1 = bottom;

                        let new_bottom_seed = top;
                        let new_top_seed = top_seed;
                        seeds.push((new_bottom_seed,new_top_seed,false));
                    }
                    (Equal,Greater) | (Greater,Greater) => {
                        let new_bottom_seed = top;
                        let new_top_seed = top_seed;
                        seeds.push((new_bottom_seed,new_top_seed,false));

                        seeds[i].0 = new_first + bottom_seed - bottom;
                        seeds[i].1 = new_first + range;
                        seeds[i].2 = true;
                    }

                }
            }

        }
    }
    let mut first = seeds.iter().map(|x| x.0).collect::<Vec<u64>>();
    first.sort();
    first.first().unwrap().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("Example.txt");
        assert_eq!(part1_implementation(input), 35.to_string());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("Example.txt");
        assert_eq!(part2_implementation(input), 46.to_string());
    }
}