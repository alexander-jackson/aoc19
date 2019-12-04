fn get_digits(input: u32) -> Vec<u32> {
    input.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}

fn main() {
    dbg!(get_digits(114567));
}

#[test]
fn get_digits_test() {
    assert_eq!(
        get_digits(114567),
        vec![1, 1, 4, 5, 6, 7]
    );
}
