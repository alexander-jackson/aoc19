use std::fs;
use std::io;

#[derive(Debug, PartialEq)]
struct Op {
    a: i8,
    b: i8,
    c: i8,
    d: i8,
}

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

/// Splits an opcode up into an Op struct
fn split_opcode(opcode: i32) -> Op {
    let op_str: String = format!("{:0>5}", opcode.to_string());
    let mut op_it = op_str.chars();

    Op {
        a: op_it.next().unwrap().to_digit(10).unwrap() as i8,
        b: op_it.next().unwrap().to_digit(10).unwrap() as i8,
        c: op_it.next().unwrap().to_digit(10).unwrap() as i8,
        d: op_it.collect::<String>().parse().unwrap(),
    }
}

fn add_op(input: &mut Vec<i32>, index: &mut i32, op: Op) {
    let uindex: usize = *index as usize;

    // Add instruction
    let c: i32 = input[uindex + 1];
    let b: i32 = input[uindex + 2];
    let dest: i32 = input[uindex + 3];

    if op.a == 0 {
        // Positional
        input[dest as usize] =
            if op.c == 0 { input[c as usize] } else { c }
            +
            if op.b == 0 { input[b as usize] } else { b }
    } else {
        // Immediate addressing
    }

    *index += 4;
}

fn multiply_op(input: &mut Vec<i32>, index: &mut i32, op: Op) {
    let uindex: usize = *index as usize;

    // Multiply instruction
    let c: i32 = input[uindex + 1];
    let b: i32 = input[uindex + 2];
    let dest: i32 = input[uindex + 3];

    if op.a == 0 {
        // Positional
        input[dest as usize] =
            if op.c == 0 { input[c as usize] } else { c }
            *
            if op.b == 0 { input[b as usize] } else { b }
    } else {
        // Immediate addressing
    }

    *index += 4;
}

fn read_op(input: &mut Vec<i32>, index: &mut i32) {
    let uindex: usize = *index as usize;

    let mut line: String = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let dest: i32 = input[uindex + 1];

    input[dest as usize] = line.trim().parse().unwrap();

    *index += 2;
}

fn output_op(input: &mut Vec<i32>, index: &mut i32) {
    let uindex: usize = *index as usize;

    let value: i32 = input[uindex + 1];
    println!("Output instruction: {}", input[value as usize]);

    *index += 2;
}

fn perform_opcode(input: &mut Vec<i32>, index: &mut i32, opcode: i32) {
    // Split the opcode up
    let op: Op = split_opcode(opcode);

    match op.d {
        1 => add_op(input, index, op),
        2 => multiply_op(input, index, op),
        3 => read_op(input, index),
        4 => output_op(input, index),
        _ => panic!("Invalid opcode"),
    }
}

fn execute(input: &mut Vec<i32>) {
    let mut index: i32 = 0;

    loop {
        // Get opcode, a, b and dest
        let opcode: i32 = input[index as usize];

        if opcode == 99 {
            break;
        }

        perform_opcode(input, &mut index, opcode);
    }
}

fn main() {
    let filename: &str = "input.txt";

    let mut values: Vec<i32> = get_values(filename);

    execute(&mut values);
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

#[test]
fn split_opcode_test() {
    assert_eq!(split_opcode(1002), Op { a: 0, b: 1, c: 0, d: 2 });
}

#[test]
fn multiplication_test() {
    let mut values: Vec<i32> = format_input("1002,4,3,4,33");

    execute(&mut values);

    assert_eq!(values, format_input("1002,4,3,4,99"));
}

#[test]
fn addition_test() {
    let mut values: Vec<i32> = format_input("1001,4,66,4,33");

    execute(&mut values);

    assert_eq!(values, format_input("1001,4,66,4,99"));
}
