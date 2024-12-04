use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let level = parse_list_from_file("input/day04.txt")?;

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
    let n = level.len();
    let m = level[0].len();
    for x in 0..n {
        for y in 0..m {
            let ri = level[x].get(y..y+4).unwrap_or_default();
            if ri == "XMAS" || ri == "SAMX"{     
                result += 1;
            }
            let mut dn = String::from("");
            for p in 0..4 {
                if p+y >= n {
                    continue
                }
                dn = dn + level[p+y].get(x..x+1).unwrap_or_default();
            }
            if dn == "XMAS" || dn == "SAMX" {
                result += 1;
            }
            let mut du = String::from("");
            for p in 0..4{
                if p+x >= m || 4+y-p <= 3{
                    continue
                }
                du += level[y-p].get(p+x..p+x+1).unwrap_or_default();
            }
            if du.as_str() == "XMAS" || du.as_str() == "SAMX" {
                result += 1;
            }
            let mut dd = String::from("");
            for p in 0..4{
                if p+x >= m || p+y >= n {
                    continue
                }
                dd += level[y+p].get(p+x..p+x+1).unwrap_or_default();
            }
            if dd.as_str() == "XMAS" || dd.as_str() == "SAMX" {    
                result += 1;
            }
        }
    }
    result
}

fn part2(input: &Vec<String>) -> i32 {
    let mut result:i32 = 0;
    let x_range = input[0].len();
    let y_range = input.len();
    
    for y in 1..y_range-1{
        for x in 1..x_range-1{
            if input[y].get(x..x+1).unwrap_or_default() != "A"{
                continue
            }
            let (q1, q2, q3, q4) = (
                input[y-1].get(x-1..x).unwrap_or_default(), 
                input[y-1].get(x+1..x+2).unwrap_or_default(), 
                input[y+1].get(x-1..x).unwrap_or_default(), 
                input[y+1].get(x+1..x+2).unwrap_or_default()
                );
            if (q1 == "M" && q4 == "S") || (q1 == "S" && q4 == "M") {
                if (q2 == "M" && q3 == "S") || (q2 == "S" && q3 == "M"){
                    result += 1;
                }
            }
        }
    }

    result
}

