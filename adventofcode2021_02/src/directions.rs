#[derive(PartialEq, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub fn dive_instructions(vec: &[(String, i32)]) -> Coordinate {
    let mut result = Coordinate {x: 0, y: 0};
    for (direction, distance) in vec {
        match direction.as_str() {
           "forward" => result.x += distance,
           "backward" => result.x -= distance,
           "up" => result.y -= distance,
           "down" => result.y += distance,
           _ => println!("unexpected direction: {}", direction)
        }
    }
    result
}

pub fn horizontal_by_depth(coords: &Coordinate) -> i32 {
    coords.x * coords.y
}

#[cfg(test)]
mod tests {
    use crate::directions::Coordinate;

    use super::dive_instructions;

    #[test]
    fn test_dive_instruction_simple_forward() {
        let v = vec![(String::from("forward"), 2), (String::from("forward"), 3)];
        let expected_result = Coordinate{ x: 5, y: 0};
        assert_eq!(expected_result, dive_instructions(&v));
    }

    #[test]
    fn test_dive_instruction_simple_forward_and_down() {
        let v = vec![(String::from("forward"), 2), (String::from("forward"), 3), (String::from("down"), 3)];
        let expected_result = Coordinate{ x: 5, y: 3};
        assert_eq!(expected_result, dive_instructions(&v));
    }

}