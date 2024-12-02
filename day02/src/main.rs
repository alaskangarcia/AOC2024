use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let level = parse_list_from_file("input/day02.txt")?;

    println!("Part 1: {}", part1(&level));
    println!("Part 2: {}", part2(&level));
   
   Ok(())
}


fn parse_list_from_file(fname: &str) -> std::io::Result<Vec<Vec<i32>>> {
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
        let nums = line.split_whitespace().map(|num| num.parse::<i32>()
            .unwrap()).collect::<Vec<i32>>();
        level.push(nums);
    }
    Ok(level)
}

fn part1(level: &Vec<Vec<i32>>) -> i32{
    let mut result: i32 = 0;
    for lvl in level{
        if check(&lvl) {
            result = result +1;
        }
    }
    result
}

fn check(lvl: &Vec<i32>) -> bool{
    let mut res = false;
    let mut sorte = lvl.clone();
    sorte.sort();
    let mut sortr = sorte.clone();
    sortr.reverse();
    if *lvl == sorte || *lvl == sortr{
        res = true
    }
    for l in 0..lvl.len()-1{
        if (lvl[l]-lvl[l+1]).abs() > 3 || (lvl[l]-lvl[l+1]).abs() < 1 {
            res = false;
        }
    }
    res
}

fn part2(level: &Vec<Vec<i32>>) -> i32{
    let mut result: i32 = 0;
    for lvl in level{   
        for i in 0..lvl.len() {
            let mut a = lvl.clone();
            a.remove(i);
            if check(&a) {
                result = result + 1;
                break;
            }
        }
    }
    result
}
