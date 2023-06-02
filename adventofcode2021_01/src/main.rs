use adventofcode2021_01::depth::{count_true_values, next_is_bigger, sum_three_window_to_vec};
use adventofcode2021_01::file::load_number_txt_to_vector;
use std::env;

fn main() {
    // CLI fileinputs
    let filename_args: Vec<String> = env::args().collect();

    // Part One
    let number_vec = load_number_txt_to_vector(&filename_args[1]).unwrap();
    // Check pairs of values increasing
    let is_bigger_vec = next_is_bigger(&number_vec);
    // Only count increasing
    let count_increases = count_true_values(&is_bigger_vec);
    println!("{:?}", count_increases);

    // Part Two
    let number_vec = load_number_txt_to_vector(&filename_args[2]).unwrap();
    // Chunks of 3
    let sum_windows = sum_three_window_to_vec(&number_vec);
    println!("{:?}", sum_windows);
    // Check pairs of values increasing
    let is_bigger_vec = next_is_bigger(&sum_windows);
    // Only count increasing
    let count_increases = count_true_values(&is_bigger_vec);
    println!("{:?}", count_increases);
}
