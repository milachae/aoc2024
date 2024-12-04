use std::fs;

fn read_file(file: &str) -> Vec<Vec<char>>{
    let content = fs::read_to_string(file).unwrap();
    content.split("\n").map(|x1| x1.chars().collect()).collect()
}

fn horizontal(text: &Vec<Vec<char>>)-> i32 {
    let mut count: i32 = 0;
    for row in text {
        for col in 0..row.len()-3 {
            let t: String = row[col..col +4].iter().collect();
            if t == "XMAS" || t == "SAMX" {
                count += 1;
            }
        }
    }
    count
}

fn vertical(text: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    for col in 0..text[0].len() {
        for row in 0..text.len()-3 {
            let t = format!("{}{}{}{}", text[row][col], text[row+1][col], text[row+2][col], text[row+3][col]);
            if t == "XMAS" || t == "SAMX" {
                count += 1;
            }
        }
    }
    count
}

fn diagonal(text: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;

    for row in 0..text.len()-3 {
        for col in 0..text[row].len() {
            if col < text[row].len()-3 {
                let t = format!("{}{}{}{}", text[row][col], text[row+1][col+1], text[row+2][col+2], text[row+3][col+3]);
                if t == "XMAS" || t == "SAMX" {
                    count += 1;
                }
            }

            if col >= 3 {
                let t = format!("{}{}{}{}", text[row][col], text[row+1][col-1], text[row+2][col-2], text[row+3][col-3]);
                if t == "XMAS" || t == "SAMX" {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part1() -> i32 {
    let text: Vec<Vec<char>> = read_file("4_1.txt");
     horizontal(&text) + vertical(&text) + diagonal(&text)
}

fn part2() -> i32 {
    let text: Vec<Vec<char>> = read_file("4_2.txt");
    let mut count: i32 = 0;

    for row in 0..text.len()-2 {
        for col in 0..text[row].len()-2 {
            let t1 = format!("{}{}{}", text[row][col], text[row+1][col+1], text[row+2][col+2]);
            let t2 = format!("{}{}{}", text[row][col+2], text[row+1][col+1], text[row+2][col]);

            if (t1 == "MAS" || t1 == "SAM") && (t2 == "MAS" || t2 == "SAM") {
                count += 1;
            }
        }
    }
    count
}



fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
