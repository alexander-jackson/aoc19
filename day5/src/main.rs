use std::fs;

fn format_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_values(filename: &str) -> Vec<i32> {
    format_input(&fs::read_to_string(filename).expect("Failed to read the problem input"))
}

fn execute(input: &mut Vec<i32>) {
    let mut index: i32 = 0;

    loop {
        // Get opcode, a, b and dest
        let opcode: i32 = input[index as usize];

        if opcode == 99 {
            break;
        }

        let a: i32 = input[index as usize + 1];
        let b: i32 = input[index as usize + 2];
        let dest: i32 = input[index as usize + 3];

        if opcode == 1 {
            input[dest as usize] = input[a as usize] + input[b as usize];
        } else if opcode == 2 {
            input[dest as usize] = input[a as usize] * input[b as usize];
        } else {
            panic!("Something went wrong");
        }

        index += 4;
    }
}

fn test_noun_and_verb(filename: &str, noun: i32, verb: i32) -> bool {
    // Read the file
    let mut values: Vec<i32> = get_values(filename);

    values[1] = noun;
    values[2] = verb;

    execute(&mut values);

    values[0] == 19690720
}

fn main() {
    let filename: &str = "input.txt";

    let mut noun: i32 = 0;
    let mut verb: i32 = 0;

    while !test_noun_and_verb(filename, noun, verb) {
        verb += 1;

        if verb == 100 {
            noun += 1;
            verb = 0;
        }
    }

    dbg!(noun * 100 + verb);
}

#[test]
fn first_test() {
    let mut values: Vec<i32> = format_input("1,0,0,0,99");

    execute(&mut values);

    assert_eq!(values, format_input("2,0,0,0,99"));
}

#[test]
fn second_test() {
    let mut values: Vec<i32> = format_input("2,3,0,3,99");

    execute(&mut values);

    assert_eq!(values, format_input("2,3,0,6,99"));
}

#[test]
fn third_test() {
    let mut values: Vec<i32> = format_input("2,4,4,5,99,0");

    execute(&mut values);

    assert_eq!(values, format_input("2,4,4,5,99,9801"));
}

#[test]
fn fourth_test() {
    let mut values: Vec<i32> = format_input("1,1,1,4,99,5,6,0,99");

    execute(&mut values);

    assert_eq!(values, format_input("30,1,1,4,2,5,6,0,99"));
}
