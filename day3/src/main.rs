use std::fs;
use regex::Regex;

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}
fn parse_mul(content: &String) -> Vec<(i32, i32, i32)> {

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    re.captures_iter(&*content).map(|caps| {
        let (_, [el1, el2]) = caps.extract();
        (el1.parse().unwrap(), el2.parse().unwrap(), caps.get(0).unwrap().start() as i32)
    }).collect()
}

fn parse_do_dont(content:&String) -> (Vec<i32>, Vec<i32>) {
    let re1 = Regex::new(r"do\(\)").unwrap();
    let re2 = Regex::new(r"don't\(\)").unwrap();

    let m1 = re1.find_iter(&*content).map(|m| m.start() as i32).collect();
    let m2 = re2.find_iter(&*content).map(|m| m.start() as i32).collect();
    (m1,m2)
}

fn part1() -> i32 {

    let mut result: i32 = 0;

    for m in parse_mul(&read_file("3_1.txt")) {
        result += m.0*m.1;
    }
    result
}

fn is_allowed(i: &i32, d: &Vec<i32>, dt: &Vec<i32>) -> bool {
    let mut d_i = 0;
    let mut dt_i = 0;

    while d_i < d.len() && d[d_i] < *i {
        d_i += 1;
    }
    while dt_i < d.len() && dt[dt_i] < *i {
        dt_i += 1;
    }

    if dt_i == 0 {
        return true
    } else if dt_i > 0 && d_i == 0 {
        return false
    }
    d[d_i-1] > dt[dt_i-1]
}

fn part2() -> i32 {

    let file = read_file("3_2.txt");
    let mul = parse_mul(&file);
    let d = parse_do_dont(&file);
    let mut result: i32 = 0;
    for m in mul {
        if is_allowed(&m.2, &d.0, &d.1) {
            result += m.0*m.1;
        }
    }
    result
}


fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
