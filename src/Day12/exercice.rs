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
        let map = split[0];

        let repartition = split[1]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let repartition = repartition.iter().rev().cloned().collect::<Vec<i32>>();

        let arrangement = get_arrangement(map, &repartition,0);
        let arrangement : Vec<&Vec<i32>> = arrangement.iter().filter(|(_,x)| x.iter().filter(|&&x| x!=0).map(|&x| x).collect::<Vec<i32>>() == repartition).map(|(_, x)| x).collect();
        sum += arrangement.iter().count();
    }
    sum.to_string()
}
pub fn part2_implementation(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        println!("{}",line);
        let split = line.split(" ").collect::<Vec<&str>>();

        let new_map = vec![split[0]; 5].join("?");
        let new_repartition = vec![split[1]; 5].join(",");

        let map = new_map.as_str();
        let repartition = new_repartition
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let repartition = repartition.iter().rev().cloned().collect::<Vec<i32>>();
        let arrangement = get_arrangement(map, &repartition,0);
        let arrangement : Vec<&Vec<i32>> = arrangement.iter().filter(|(_,x)| x.iter().filter(|&&x| x!=0).map(|&x| x).collect::<Vec<i32>>() == repartition).map(|(_, x)| x).collect();
        sum += arrangement.iter().count();
    }
    sum.to_string()
}
fn get_arrangement(mapping: &str, repartition: &Vec<i32>, index : usize) -> Vec<(usize,Vec<i32>)> {

    if mapping.len() == 0 || repartition.len() <= index {
        return vec![];
    }

    let mut sum = 0;
    let mut count = 0;
    let mut new = true;

    let char = mapping.chars().next().unwrap();

    let next = &mapping[1..];
    let mut next_possible = get_arrangement(next, &repartition,index);
     match char {
        '#' => {
            next_possible = do_good_pipe( &repartition,  &next_possible);
        }
        '.' => {
            next_possible = do_bad_pipe(&repartition, &next_possible);
            }

        '?' => {
            //add Clones
            let mut next_possible_clone = next_possible.clone();
            next_possible = do_good_pipe(&repartition, &next_possible);
            next_possible_clone = do_bad_pipe(&repartition, &next_possible_clone);
            next_possible.append(&mut next_possible_clone);
        }
        _ => panic!("Invalid char")
    }
    next_possible
}

fn do_bad_pipe(repartition: &Vec<i32>, next_possible: &Vec<(usize, Vec<i32>)>) -> Vec<(usize, Vec<i32>)>{
    if next_possible.len() == 0 {
        return vec![(0,vec![0])];
    }
    let mut temp = vec![];
    for (index,elem) in next_possible.iter() {
        if *elem.last().unwrap() == 0 {
            temp.push((*index,elem.clone()));
        }
        else if index < &repartition.len() && *elem.last().unwrap() == repartition[*index] {
            let mut clone = elem.clone();
            clone.push(0);
            temp.push((*index+1,clone));
        }

    }
    temp
}

fn do_good_pipe(repartition: &Vec<i32>, next_possible: &Vec<(usize, Vec<i32>)>) ->  Vec<(usize, Vec<i32>)>{
    if next_possible.len() == 0 {
        return vec![(0,vec![1])];
    }
    let mut temp = vec![];
    for (index,elem) in next_possible.iter() {
        let mut clone = elem.clone();
        if *elem.last().unwrap() == 0 {
            clone.push( 1);
            temp.push((*index,clone));
        } else {
            if index < &repartition.len() && *elem.last().unwrap() < repartition[*index] {
                let mut last = clone.last_mut().unwrap();
                *last += 1;
                temp.push((*index,clone));
            }
        }
    }
    temp
}

fn get_repartition_spring(list: &Vec<i32>) -> Vec<i32> {
    let mut repartition = vec![];
    let mut count = 0;
    for &elem in list.iter() {
        if elem == 0 {
            if count != 0 {
                repartition.push(count);
                count = 0;
            }
        } else {
            count += 1;
        }
    }
    if count != 0 {
        repartition.push(count);
    }
    repartition
}

fn get_spring(chain: &str) -> Vec<Vec<i32>> {
    match chain.chars().next() {
        Some(char) => {
            let next = &chain[1..];
            let mut next_possible = get_spring(next);
            if char == '#' {
                for elem in next_possible.iter_mut() {
                    elem.insert(0, 1);
                }
            }

            if char == '?' {
                //add Clones
                let mut next_possible_clone = next_possible.clone();

                for elem in next_possible_clone.iter_mut() {
                    elem.insert(0, 0);
                }

                for elem in next_possible.iter_mut() {
                    elem.insert(0, 1);
                }

                next_possible.append(&mut next_possible_clone);
            }
            next_possible
        }
        None => vec![vec![]],
    }
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
