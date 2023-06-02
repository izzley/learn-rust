#[derive(PartialEq, Debug)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

pub fn dive_instructions(vec: &[(String, i64)]) -> Coordinate {
    let mut result = Coordinate { x: 0, y: 0 };
    for (direction, distance) in vec {
        match direction.as_str() {
            "forward" => result.x += distance,
            "up" => result.y -= distance,
            "down" => result.y += distance,
            _ => println!("unexpected direction: {}", direction),
        }
    }
    result
}

pub fn dive_instructions_aim(vec: &[(String, i64)]) -> Coordinate {
    // Part two includes aim calibration for depth. Depth only affected by forward
    let mut result = Coordinate { x: 0, y: 0 };
    let mut aim: i64 = 0;
    for (direction, distance) in vec {
        match direction.as_str() {
            "forward" => {
                result.x += distance;
                // Increases depth by aim multiplied by distance
                result.y += aim * distance;
            }
            "up" => {
                aim -= distance;
            }
            "down" => {
                aim += distance;
            }
            _ => {
                println!(
                    "unexpected direction: {} with step: {}",
                    direction, distance
                )
            }
        }
    }
    result
}
pub fn horizontal_by_depth(coords: &Coordinate) -> i64 {
    coords.x * coords.y
}

#[cfg(test)]
mod tests {
    use crate::directions::Coordinate;

    use super::dive_instructions;

    #[test]
    fn test_dive_instruction_simple_forward() {
        let v = vec![(String::from("forward"), 2), (String::from("forward"), 3)];
        let expected_result = Coordinate { x: 5, y: 0 };
        assert_eq!(expected_result, dive_instructions(&v));
    }

    #[test]
    fn test_dive_instruction_simple_forward_and_down() {
        let v = vec![
            (String::from("forward"), 2),
            (String::from("forward"), 3),
            (String::from("down"), 3),
        ];
        let expected_result = Coordinate { x: 5, y: 3 };
        assert_eq!(expected_result, dive_instructions(&v));
    }
}
