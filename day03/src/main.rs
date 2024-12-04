use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
fn main() -> std::io::Result<()> {
    let level = parse_list_from_file("input/day03.txt")?;

    println!("Part 1: {}", part1(&level));
    println!("Part 2: {}", part2(&level));
   
   Ok(())
}


fn parse_list_from_file(fname: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);
    let lines = reader.lines();
    let mut level = Vec::new();
    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        level.push(line);
    }
    Ok(level)
}

fn part1(level: &Vec<String>) -> i32{
    let mut result: i32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for lvl in level {
        for (_, [x,y]) in re.captures_iter(lvl.as_str()).map(|caps| caps.extract()){
            result += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
        }    
    }
    result
}

fn part2(level: &Vec<String>) -> i32{
    let mut result: i32 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)(?:n't)?\(\)").unwrap();
    let mut enabled = true;
    for lvl in level {
        for (s, [x,y]) in re.captures_iter(lvl.as_str()).map(|caps| caps.extract()){
            match s {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => result += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap() * enabled as i32,      
            };
        }    
    }
    result
}
