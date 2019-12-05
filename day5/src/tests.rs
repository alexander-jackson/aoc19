use super::*;

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
    assert_eq!(
        split_opcode(1002),
        Op {
            a: 0,
            b: 1,
            c: 0,
            d: 2
        }
    );
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

#[test]
fn less_than_test() {
    let mut values: Vec<i32> = format_input("7,1,2,3");

    execute(&mut values);

    assert_eq!(values, format_input("7,1,2,1"));
}

#[test]
fn equals_test() {
    let mut values: Vec<i32> = format_input("8,1,1,3");

    execute(&mut values);

    assert_eq!(values, format_input("8,1,1,1"));
}
