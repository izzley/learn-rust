use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_directions_txt_to_vec_tuple(path: &String) -> Result<Vec<(String, i32)>, std::io::Error> {
    let mut direction_steps = Vec::<(String, i32)>::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.split(' ').collect();
        let num = parts[1].parse::<i32>().unwrap();
        let result = (String::from(parts[0]), num);
        direction_steps.push(result);
        }
    return Ok(direction_steps);
    }
