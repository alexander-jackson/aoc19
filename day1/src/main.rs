use std::fs;

fn part_one() {
    let result: i64 = fs::read_to_string("input.txt")
        .expect("Failed to read the file")
        .split('\n')
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
        .map(|v: i64| v / 3 - 2)
        .sum();

    dbg!(result);
}

fn get_fuel_requirement(mass: i64) -> i64 {
    // Calculate the fuel requirement
    let req: i64 = mass / 3 - 2;

    if req <= 0 {
        return 0;
    }

    req + get_fuel_requirement(req)
}

fn part_two() {
    let result: i64 = fs::read_to_string("input.txt")
        .expect("Failed to read the file")
        .split('\n')
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
        .map(|v: i64| get_fuel_requirement(v))
        .sum();

    dbg!(result);
}

fn main() {
    part_one();
    part_two();
}

#[test]
fn get_fuel_requirement_test() {
    let masses: Vec<i64> = vec![14, 1969, 100756];
    let reqs: Vec<i64> = masses.iter().map(|v| get_fuel_requirement(*v)).collect();
    let expected: Vec<i64> = vec![2, 966, 50346];

    assert_eq!(reqs, expected);
}
