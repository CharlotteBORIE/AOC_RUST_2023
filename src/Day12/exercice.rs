pub fn part1(){
    let input = include_str!("Input.txt");
    println!("Day 12 Part 1");
    println!("{:?}", part1_implementation(input));
}

pub fn part2(){
    let input = include_str!("Input.txt");
    println!("Day 12 Part 2");
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
        let split = line.split(" ").collect::<Vec<&str>>();
        let map = split[0].split(".").filter(|x| !x.is_empty()).collect::<Vec<&str>>();

        let repartition = split[1].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        sum += get_arrangement(map, repartition);
    }
    sum.to_string()
}
pub fn part2_implementation(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<&str>>();
        
        let new_map = vec![split[0];5].join("?");
        let new_repartition = vec![split[1];5].join(",");
        
        let map = new_map.split(".").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
        let repartition = new_repartition.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sum += get_arrangement(map, repartition);
    }
    sum.to_string()
}
fn get_arrangement(mapping: Vec<&str>, repartition: Vec<i32>) -> usize {
    let mut sum = 0;
    let mut full_possible = Vec::new();
    for elem in mapping.iter() {
        let possible_springs = get_spring(*elem).iter().map(|x| get_repartition_spring(x)).collect::<Vec<Vec<i32>>>();
        if full_possible.is_empty() {
            full_possible = possible_springs;
        }
        else {
            let mut new_full_possible = Vec::new();
            for elem in full_possible.iter() {
                for elem2 in possible_springs.iter() {
                    let mut new_elem = elem.clone();
                    new_elem.append(&mut elem2.clone());
                    new_full_possible.push(new_elem);
                }
            }
            full_possible = new_full_possible;
        }
    }
    let filter =full_possible.iter().filter(|possible| possible==&&repartition);
    filter.count()
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
        }
        else {
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
                    elem.insert(0,1);
                }
            }

            if char == '?' {
                //add Clones
                let mut next_possible_clone = next_possible.clone();

                for elem in next_possible_clone.iter_mut() {
                    elem.insert(0,0);
                }

                for elem in next_possible.iter_mut() {
                    elem.insert(0,1);
                }

                next_possible.append(&mut next_possible_clone);
            }
            next_possible
        },
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