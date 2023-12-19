use std::fs::File;
use std::i32;
use std::io::Read;

fn partOne(content: &str) -> i32 {
    let mut total = 0;

    for line in content.lines() {
        for c in line.chars() {
            if c.is_numeric() {
                total += c.to_string().parse::<i32>().unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                total += c.to_string().parse::<i32>().unwrap();
                break;
            }
        }
    }

    return total;
}

fn partTwo(content: &str) -> i32 {
    let mut total = 0;
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in content.lines() {
        'first: for (i, c) in line.char_indices() {
            if c.is_numeric() {
                total += c.to_string().parse::<i32>().unwrap() * 10;
                break;
            }
            for j in 0..numbers.len() {
                if line[i..].starts_with(numbers[j]) {
                    total += (j + 1) as i32 * 10;
                    break 'first;
                }
            }
        }
        'second: for (i, c) in line.char_indices().rev() {
            if c.is_numeric() {
                total += c.to_string().parse::<i32>().unwrap();
                break;
            }
            for j in 0..numbers.len() {
                if line[i..].starts_with(numbers[j]) {
                    total += (j + 1) as i32;
                    break 'second;
                }
            }
        }
    }

    return total;
}

fn main() {
    let mut file = File::open("src/part-two.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    // println!("THE TOTAL IS {}", partOne(&content));
    println!("THE TOTAL IS {}", partTwo(&content));
}
