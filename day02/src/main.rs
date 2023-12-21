use std::{collections::HashMap, fs::File, io::Read};

fn part_one(content: &str) -> i32 {
    let mut max_cube: HashMap<&str, i32> = HashMap::new();
    max_cube.insert("red", 12);
    max_cube.insert("green", 13);
    max_cube.insert("blue", 14);

    let mut total = 0;

    for line in content.lines() {
        let mut parts: Vec<&str> = line.trim().split(":").collect();
        let game_id = parts[0].strip_prefix("Game ").unwrap();
        let mut all_ok = true;

        parts = parts[1].split(";").collect();

        'first: for part in parts {
            let cubes: Vec<&str> = part.trim().split(",").collect();
            for cube in cubes {
                let cube_vec: Vec<&str> = cube.trim().split(" ").collect();
                if let Some(value) = max_cube.get(cube_vec[1]) {
                    if &cube_vec[0].to_string().parse::<i32>().unwrap() > value {
                        all_ok = false;
                        break 'first;
                    }
                }
            }
        }

        if all_ok {
            total += game_id.to_string().parse::<i32>().unwrap();
        }
    }

    return total;
}

fn part_two(content: &str) -> i32 {
    let mut total = 0;

    for line in content.lines() {
        let mut fewest_cubes: HashMap<String, i32> = HashMap::new();

        let mut parts: Vec<&str> = line.trim().split(":").collect();
        parts = parts[1].split(";").collect();

        for part in parts {
            let cubes: Vec<&str> = part.trim().split(",").collect();
            for cube in cubes {
                let cube_vec: Vec<&str> = cube.trim().split(" ").collect();
                let cube_count = cube_vec[0].to_string().parse::<i32>().unwrap();

                if let Some(value) = fewest_cubes.get(cube_vec[1]) {
                    if value < &cube_count {
                        fewest_cubes.insert(cube_vec[1].to_string(), cube_count);
                    }
                } else {
                    fewest_cubes.insert(cube_vec[1].to_string(), cube_count);
                }
            }
        }

        let mut total_mul = 1;
        for (_, v) in fewest_cubes {
            total_mul *= v;
        }

        total += total_mul;
    }

    return total;
}

fn main() {
    let mut file = File::open("src/input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    println!("TOTAL {}", part_one(&content));
    println!("TOTAL {}", part_two(&content));
}
