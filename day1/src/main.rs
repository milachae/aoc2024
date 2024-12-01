use std::{fs};

fn read_file(file: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(file).expect("a");
    let v: Vec<&str> = contents.split("\n").collect();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for x in v {
        let vv: Vec<&str> = x.split_whitespace().collect();
        if vv.len() == 2 {
            left.push((&vv[0]).parse().unwrap());
            right.push((&vv[1]).parse().unwrap());
        }
    }
    (left, right)
}
fn part1() -> i32 {

    let (mut left, mut right) = read_file("1_1.txt");
    left.sort();
    right.sort();

    let mut result: i32 = 0;
    for i in 0..left.len() {
        result += (left[i] - right[i]).abs();
    }
    result
}

fn part2 () -> i32 {
    let (left, right) = read_file("1_2.txt");

    let mut result: i32 = 0;

    for i in 0..left.len() {
        result += left[i] * right.iter().filter(|&n| n == &left[i]).count() as i32;
    }

    result
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
