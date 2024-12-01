use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let (mut left_list, mut right_list) = parse_lists_from_file("input/day01.txt")?;
    left_list.sort();
    right_list.sort();

    println!("Part1 sum: {}", part1(&left_list, &right_list));
    println!("Part2 sum: {}", part2(&left_list, &right_list));

    Ok(())
}

fn parse_lists_from_file(fname: &str) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let lines = reader.lines();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        let nums = line.split_whitespace().map(|num| num.parse::<i32>()
            .unwrap()).collect::<Vec<i32>>();
        left_list.push(nums[0]);
        right_list.push(nums[1]);
    }

    Ok((left_list, right_list))
}

fn part1(left_list: &[i32], right_list: &[i32]) -> i32{
    let mut i = 0;
    let z = left_list.len();
    let mut result: i32 = 0;
    while i < z{
        result = result + (left_list[i] - right_list[i]).abs();
        i = i + 1;
    } 
    result
}

fn part2(left_list: &[i32], right_list: &[i32]) -> i32{
    let mut res: i32 = 0;

    for i in 1..left_list.len(){
        res = res + (right_list.into_iter()
            .filter(|&x| *x == left_list[i])
            .count() as i32 
            * left_list[i]);

    }
    res
}
