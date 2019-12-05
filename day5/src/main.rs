use std::io;
use std::fs;
use std::env;

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

fn jump_if_true_op(input: &mut Vec<i32>, index: &mut i32, op: Op) {
    let uindex: usize = *index as usize;

    let mut cond: i32 = input[uindex + 1];
    let mut ptr: i32 = input[uindex + 2];

    cond = if op.c == 0 { input[cond as usize] } else { cond };
    ptr = if op.b == 0 { input[ptr as usize] } else { ptr };

    *index = if cond != 0 { ptr } else { *index + 3 };
}

fn jump_if_false_op(input: &mut Vec<i32>, index: &mut i32, op: Op) {
    let uindex: usize = *index as usize;

    let mut cond: i32 = input[uindex + 1];
    let mut ptr: i32 = input[uindex + 2];

    cond = if op.c == 0 { input[cond as usize] } else { cond };
    ptr = if op.b == 0 { input[ptr as usize] } else { ptr };

    *index = if cond == 0 { ptr } else { *index + 3 };
}

fn less_than_op(input: &mut Vec<i32>, index: &mut i32, op: Op) {
    let uindex: usize = *index as usize;

    let mut first: i32 = input[uindex + 1];
    let mut second: i32 = input[uindex + 2];
    let third: i32 = input[uindex + 3];

    // Convert first and second if needed
    first = if op.c == 0 { input[first as usize] } else { first };
    second = if op.b == 0 { input[second as usize] } else { second };

    input[third as usize] = if first < second { 1 } else { 0 };

    *index += 4;
}

fn equals_op(input: &mut Vec<i32>, index: &mut i32, op: Op) {
    let uindex: usize = *index as usize;

    let mut first: i32 = input[uindex + 1];
    let mut second: i32 = input[uindex + 2];
    let third: i32 = input[uindex + 3];

    // Convert first and second if needed
    first = if op.c == 0 { input[first as usize] } else { first };
    second = if op.b == 0 { input[second as usize] } else { second };

    input[third as usize] = if first == second { 1 } else { 0 };

    *index += 4;
}

fn perform_opcode(input: &mut Vec<i32>, index: &mut i32, opcode: i32) {
    // Split the opcode up
    let op: Op = split_opcode(opcode);

    match op.d {
        1 => add_op(input, index, op),
        2 => multiply_op(input, index, op),
        3 => read_op(input, index),
        4 => output_op(input, index),
        5 => jump_if_true_op(input, index, op),
        6 => jump_if_false_op(input, index, op),
        7 => less_than_op(input, index, op),
        8 => equals_op(input, index, op),
        _ => panic!("Invalid opcode"),
    }
}

fn execute(input: &mut Vec<i32>) {
    let mut index: i32 = 0;
    let mut opcode: i32 = input[index as usize];

    while opcode != 99 {
        perform_opcode(input, &mut index, opcode);
        opcode = input[index as usize];
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = if args.len() > 1 { &args[1] } else { "input.txt" };

    let mut values: Vec<i32> = get_values(filename);
    execute(&mut values);
}

#[cfg(test)]
mod tests;
