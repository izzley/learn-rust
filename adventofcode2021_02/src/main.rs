use adventofcode2021_02::directions::{dive_instructions, dive_instructions_aim};
use adventofcode2021_02::file::load_directions_txt_to_vec_tuple;
use adventofcode2021_02::logger;
use log::info;
use std::env;

fn main() {
    // configure logger
    let _ = logger::init();

    // CLI fileinputs
    let filename_args: Vec<String> = env::args().collect();
    let part_one_input = &filename_args[1];
    let part_two_input = &filename_args[2];

    // Part One
    let direction_step = load_directions_txt_to_vec_tuple(part_one_input).unwrap();
    let position = dive_instructions(&direction_step);
    info!("Horizontal * Depth = {}", position.x * position.y);

    // Part Two
    let direction_step = load_directions_txt_to_vec_tuple(part_two_input).unwrap();
    let position = dive_instructions_aim(&direction_step);
    info!("Horizontal * Depth (with aim adjustment)= {}", position.x * position.y)
}
