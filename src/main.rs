use std::fs;

fn split_string_into_array(contents: &str) -> Vec<String> {
    contents.split("\r\n").map(String::from).collect()
}

//test
fn convert_to_int_vector(string_vec: Vec<String>) -> Vec<i32> {
    string_vec
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
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

    let diff: i32 = v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("{}", diff);
}
