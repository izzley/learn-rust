pub fn next_is_bigger(_vec: &[i32]) -> Vec<bool> {
    // Zip offset list and compare adjacent vals
    let offset = _vec.iter().skip(1);
    let pairs = _vec.iter().zip(offset);
    pairs.map(|(a, b)| a < b).collect()
}

pub fn count_true_values(_vec: &[bool]) -> i32 {
    // Sum all true valuess
    _vec.iter().fold(0, |acc, b| if *b { acc + 1 } else { acc })
}

pub fn sum_three_window_to_vec(_vec: &[i32]) -> Vec<i32> {
    return _vec.windows(3).map(|w| w.iter().sum()).collect::<Vec<_>>();
}
