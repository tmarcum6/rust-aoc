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

fn part_one(v1: Vec<i32>, v2: Vec<i32>) {
    let diff: i32 = v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("{}", diff);
}

fn remove_duplicates(vec: Vec<i32>) -> Vec<i32> {
    let set: HashSet<_> = vec.into_iter().collect(); // Convert to HashSet to remove duplicates
    set.into_iter().collect() // Convert back to Vec
}

fn part_two(v3: Vec<i32>, v4: &Vec<i32>) {
    let mut sim: i32 = 0;
    let mut count: i32;

    let unique_v3 = remove_duplicates(v3);

    for i in unique_v3 {
        count = 0;
        for y in v4 {
            if i == *y {
                count += 1;
            }
        }
        sim += i * count;
    }

    print!("{}", sim);
}

fn main() {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    match fs::read_to_string("input/d11a.txt") {
        Ok(contents) => {
            let values_d11a = split_string_into_array(&contents);
            v1 = convert_to_int_vector(values_d11a);
            v1.sort();
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    match fs::read_to_string("input/d11b.txt") {
        Ok(contents) => {
            let values_d11b = split_string_into_array(&contents);
            v2 = convert_to_int_vector(values_d11b);
            v2.sort();
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    let v3 = v1.clone();
    let v4 = v2.clone();

    part_one(v1, v2);
    part_two(v3, &v4);
}
