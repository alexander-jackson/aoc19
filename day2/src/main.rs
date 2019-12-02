use std::fs;

fn format_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_values(filename: &str) -> Vec<usize> {
    format_input(&fs::read_to_string(filename).expect("Failed to read the problem input"))
}

fn execute(input: &mut Vec<usize>) {
    let mut index: usize = 0;

    loop {
        // Get opcode, a, b and dest
        let opcode: usize = input[index];

        if opcode == 99 {
            break;
        }

        let a: usize = input[index + 1];
        let b: usize = input[index + 2];
        let dest: usize = input[index + 3];

        if opcode == 1 {
            input[dest] = input[a] + input[b];
        } else if opcode == 2 {
            input[dest] = input[a] * input[b];
        } else {
            panic!("Something went wrong");
        }

        index += 4;
    }
}

fn test_noun_and_verb(filename: &str, noun: usize, verb: usize) -> bool {
    // Read the file
    let mut values: Vec<usize> = get_values(filename);

    values[1] = noun;
    values[2] = verb;

    execute(&mut values);

    values[0] == 19690720
}

fn main() {
    let filename: &str = "input.txt";

    let mut noun: usize = 0;
    let mut verb: usize = 0;

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
    let mut values: Vec<usize> = format_input("1,0,0,0,99");

    execute(&mut values);

    assert_eq!(values, format_input("2,0,0,0,99"));
}

#[test]
fn second_test() {
    let mut values: Vec<usize> = format_input("2,3,0,3,99");

    execute(&mut values);

    assert_eq!(values, format_input("2,3,0,6,99"));
}

#[test]
fn third_test() {
    let mut values: Vec<usize> = format_input("2,4,4,5,99,0");

    execute(&mut values);

    assert_eq!(values, format_input("2,4,4,5,99,9801"));
}

#[test]
fn fourth_test() {
    let mut values: Vec<usize> = format_input("1,1,1,4,99,5,6,0,99");

    execute(&mut values);

    assert_eq!(values, format_input("30,1,1,4,2,5,6,0,99"));
}
