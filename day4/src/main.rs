fn get_digits(input: u32) -> Vec<u32> {
    input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

fn check_adjacent_digits(input: &[u32]) -> bool {
    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] {
            return true;
        }
    }

    false
}

fn check_strict_adjacent_digits(input: &[u32]) -> bool {
    // Check that this is at least normally allowed
    if !check_adjacent_digits(input) {
        return false;
    }

    // We must have some adjacent digits, so ensure that the minimum length is 2
    let mut index: usize = 0;
    let mut min_len: u32 = std::u32::MAX;

    while index < 6 {
        let digit = input[index];
        let mut count: u32 = 1;
        index += 1;

        while index < 6 && input[index] == digit {
            index += 1;
            count += 1;
        }

        if min_len > count && count > 1 {
            min_len = count;
        }
    }

    min_len == 2
}

fn check_no_decreasing_digits(input: &[u32]) -> bool {
    for i in 0..input.len() - 1 {
        if input[i] > input[i + 1] {
            return false;
        }
    }

    true
}

fn check_validity(input: u32) -> bool {
    // Get the digits
    let digits = get_digits(input);

    check_strict_adjacent_digits(&digits) && check_no_decreasing_digits(&digits)
}

fn count_occurances(min: u32, max: u32) -> u32 {
    let mut occurances: u32 = 0;

    for i in min..=max {
        if check_validity(i) {
            occurances += 1;
            dbg!(i);
        }
    }

    occurances
}

fn main() {
    dbg!(count_occurances(171309, 643603));
}

#[test]
fn get_digits_test() {
    assert_eq!(get_digits(114567), vec![1, 1, 4, 5, 6, 7]);
}

#[test]
fn check_adjacent_digits_test() {
    assert!(check_adjacent_digits(&vec![1, 1]));
    assert!(check_adjacent_digits(&vec![1, 1, 2]));
    assert!(check_adjacent_digits(&vec![1, 2, 2]));

    assert!(!check_adjacent_digits(&vec![2, 1]));
}

#[test]
fn check_strict_adjacent_digits_test() {
    assert!(check_strict_adjacent_digits(&vec![1, 1, 2, 2, 3, 3]));
    assert!(check_strict_adjacent_digits(&vec![1, 1, 1, 1, 2, 2]));

    assert!(!check_strict_adjacent_digits(&vec![1, 2, 3, 4, 4, 4]));
}

#[test]
fn check_no_decreasing_digits_test() {
    assert!(check_no_decreasing_digits(&vec![1, 1]));
    assert!(check_no_decreasing_digits(&vec![1, 1, 1]));
    assert!(check_no_decreasing_digits(&vec![1, 1, 2]));

    assert!(!check_no_decreasing_digits(&vec![1, 3, 2]));
}

#[test]
fn check_validity_test() {
    assert!(check_validity(111111));
    assert!(!check_validity(223450));
    assert!(!check_validity(123789));
}
