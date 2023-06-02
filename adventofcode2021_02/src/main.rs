use adventofcode2021_02::file::load_directions_txt_to_vec_tuple;
use adventofcode2021_02::directions::dive_instructions;
use std::env;

fn main() {
    // CLI fileinputs
    let filename_args: Vec<String> = env::args().collect();

    // Part One
    let direction_step = load_directions_txt_to_vec_tuple(&filename_args[1]).unwrap();
    let position = dive_instructions(&direction_step);
    println!("{:?}",position.x * position.y)
}
