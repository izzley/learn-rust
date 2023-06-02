use std::fs::File;
use std::io::{BufRead, BufReader};

use log::info;

pub fn load_directions_txt_to_vec_tuple(
    path: &String,
) -> Result<Vec<(String, i64)>, std::io::Error> {
    let mut direction_steps = Vec::<(String, i64)>::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut counter: i32 = 0;

    for line_result in reader.lines() {
        counter += 1;
        let line = line_result?;
        let parts: Vec<&str> = line.split(' ').collect();
        let num = parts[1].parse::<i64>().unwrap();
        let result = (String::from(parts[0]), num);
        direction_steps.push(result);
    }
    info!("{} lines read from file: {}", path, counter);
    Ok(direction_steps)
}
