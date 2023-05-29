fn square(_vec: Vec<i32>) -> Vec<i32> {
    _vec.iter().map(|x| x.pow(2)).collect()
}

fn multiple_map(some_num: Option<i32>) -> Option<i32> {
    some_num
        .map(|n| n - 1) // => Some(5)
        .map(|n| n * n) // => Some(16)
}

fn next_is_bigger(_vec: &[i32]) -> Vec<bool> {
    let offset = _vec.iter().skip(1);
    let pairs = _vec.iter().zip(offset);
    let comparison = pairs.map(|(a, b)| a < b).collect();
    comparison
}

fn main() {
    // Example of mapping x to its power
    let v: Vec<i32> = [1, 2, 3, 4].to_vec();

    let squared = square(v);
    assert_eq!(squared, vec![1, 4, 9, 16]);

    // Example of mapping a value of 5 => (n - 1)(n - 1) => 16
    let five_less_1_squared = multiple_map(Some(5));
    assert_eq!(five_less_1_squared, Some(16));

    let v: Vec<i32> = [1, 2, 3, 4, 5, 6, 5].to_vec();

    assert_eq!(next_is_bigger(&v), [true, true, true, true, true, false].to_vec());
}
