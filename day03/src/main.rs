use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TRANSFORMS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

fn part_two(lines: &Vec<Vec<char>>) -> i32 {
    let mut all_gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    for i in 0..lines.len() {
        let mut is_number = false;
        let mut number_index = 0;
        let mut number_len = 0;
        let mut gears: Vec<(i32, i32)> = Vec::new();

        let line = &lines[i];

        for j in 0..line.len() + 1 {
            if j == line.len() || !line[j].is_numeric() {
                if is_number {
                    if !gears.is_empty() {
                        let number = line[number_index..number_index + number_len]
                            .iter()
                            .collect::<String>();

                        gears.iter().for_each(|g| {
                            all_gears
                                .entry(*g)
                                .or_insert(Vec::new())
                                .push(number.to_string().parse::<i32>().unwrap())
                        });
                    }

                    is_number = false;
                }
            } else if line[j].is_numeric() {
                if is_number {
                    number_len += 1;
                } else {
                    is_number = true;
                    number_index = j;
                    number_len = 1;
                    gears.clear();
                }
            }

            if is_number {
                TRANSFORMS.iter().for_each(|t| {
                    let new_i = i as i32 + t[0];
                    let new_j = j as i32 + t[1];

                    if new_i >= 0
                        && new_i < lines.len() as i32
                        && new_j >= 0
                        && new_j < lines[new_i as usize].len() as i32
                    {
                        if lines[new_i as usize][new_j as usize] == '*' {
                            if !gears.contains(&(new_i, new_j)) {
                                gears.push((new_i, new_j));
                            }
                        }
                    }
                })
            }
        }
    }

    println!("{:?}", all_gears);

    let total = all_gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum::<i32>();

    total
}

fn part_one(lines: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;

    for i in 0..lines.len() {
        let mut is_number = false;
        let mut number_index = 0;
        let mut number_len = 0;
        let mut touching_symbol = false;

        let line = &lines[i];

        for j in 0..line.len() + 1 {
            if j == line.len() || !line[j].is_numeric() {
                if is_number {
                    if touching_symbol {
                        let number = line[number_index..number_index + number_len]
                            .iter()
                            .collect::<String>();

                        total += number.parse::<i32>().unwrap();
                    }

                    is_number = false;
                }
            } else if line[j].is_numeric() {
                if is_number {
                    number_len += 1;
                } else {
                    is_number = true;
                    number_len = 1;
                    number_index = j;
                    touching_symbol = false;
                }
            }

            if is_number && touching_symbol == false {
                TRANSFORMS.iter().for_each(|t| {
                    let new_i = i as i32 + t[0];
                    let new_j = j as i32 + t[1];

                    if new_i >= 0
                        && new_i < lines.len() as i32
                        && new_j >= 0
                        && new_j < lines[new_i as usize].len() as i32
                    {
                        if !lines[new_i as usize][new_j as usize].is_numeric()
                            && lines[new_i as usize][new_j as usize] != '.'
                        {
                            touching_symbol = true;
                        }
                    }
                })
            }
        }
    }

    total
}

fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    // let mut content = String::new();
    // file.read_to_string(&mut content).unwrap();

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    println!("{:?}", lines);

    let lines = lines
        .iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    println!("PART ONE {}", part_one(&lines));
    println!("PART TWO {}", part_two(&lines));
}
