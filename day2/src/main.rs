use std::fs;

fn read_file(file: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file).expect("a");
    let mut result: Vec<Vec<i32>> = Vec::new();

    let v: Vec<&str> = contents.split("\n").collect();

    for line in v {
        let vv: Vec<i32> = line.split_whitespace().map(|x| -> i32{ x.parse().unwrap() }).collect();
        result.push(vv);
    }
    result
}

fn is_increasing(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if 1 > (v[i]-v[i-1]) || (v[i]-v[i-1]) > 3 {
            return false
        }
    }
    true
}

fn is_decreasing(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if 1 > (v[i-1]-v[i]) || (v[i-1]-v[i]) > 3 {
            return false
        }
    }
    true
}

fn part1() -> i32 {
    let lines: Vec<Vec<i32>> = read_file("2_1.txt");
    let mut safe: i32 = 0;

    for line in lines {

        if is_increasing(&line) || is_decreasing(&line) {
            safe += 1;
        }

    }
    safe
}

fn part2() -> i32 {
    let lines: Vec<Vec<i32>> = read_file("2_1.txt");
    let mut safe: i32 = 0;

    for line in lines {

        let mut found: bool =  is_increasing(&line) || is_decreasing(&line);

        let mut i: usize = 0;
        while !found && i < line.len() {
            let mut new_vec = line.clone();
            new_vec.remove(i);

            found =  is_increasing(&new_vec) || is_decreasing(&new_vec);
            i += 1;
        }

        if found {
            safe += 1;
        }

    }
    safe
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
