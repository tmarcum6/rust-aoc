use core::str;
use std::collections::HashSet;
use std::{fs, i32};

fn split_string_into_array(contents: &str) -> Vec<String> {
    contents.split("\r\n").map(String::from).collect()
}

fn convert_to_int_vector(string_vec: Vec<String>) -> Vec<i32> {
    string_vec
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn remove_duplicates(v: &Vec<i32>) -> Vec<i32> {
    let set: HashSet<_> = v.iter().copied().collect();
    set.into_iter().collect()
}

fn parse_input_day_one(p: &str, sort: bool) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    match fs::read_to_string(p) {
        Ok(contents) => {
            let data = split_string_into_array(&contents);
            v = convert_to_int_vector(data);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    if sort {
        v.sort();
    }

    v
}

fn _parse_input_day_two(p: &str) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    match fs::read_to_string(p) {
        Ok(contents) => {
            let data: Vec<String> = contents.lines().map(String::from).collect();
            for d in &data {
                println!("{}", d)
            }

            v = convert_to_int_vector(data);
        }
        Err(e) => {
            eprintln!("error reading file: {}", e);
        }
    }
    v
}

fn day_one_part_one(v1: &Vec<i32>, v2: &Vec<i32>) {
    let diff: i32 = v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("{}", diff);
}

fn day_one_part_two(v1: &Vec<i32>, v2: &Vec<i32>) {
    let mut sim: i32 = 0;
    let mut count: i32;

    let unique_v1 = remove_duplicates(v1);

    for i in unique_v1 {
        count = 0;
        for y in v2 {
            if i == *y {
                count += 1;
            }
        }
        sim += i * count;
    }

    print!("{}", sim);
}

fn main() {
    let v1 = parse_input_day_one("input/d1a.txt", true);
    let v2 = parse_input_day_one("input/d1b.txt", true);
    day_one_part_one(&v1, &v2);
    day_one_part_two(&v1, &v2);
}
