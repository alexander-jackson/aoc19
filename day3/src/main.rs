use std::fs;
use std::env;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn simulate(curr: Coordinate, op: (char, i32), len: &mut i32) -> Vec<(Coordinate, i32)> {
    let (dir, dist) = op;
    let mut coords: Vec<(Coordinate, i32)> = Vec::new();

    match dir {
        'U' => {
            for i in 1..=dist {
                coords.push((Coordinate { x: curr.x, y: curr.y + i }, *len));
                *len += 1;
            }
        },
        'D' => {
            for i in 1..=dist {
                coords.push((Coordinate { x: curr.x, y: curr.y - i }, *len));
                *len += 1;
            }
        },
        'L' => {
            for i in 1..=dist {
                coords.push((Coordinate { x: curr.x - i, y: curr.y }, *len));
                *len += 1;
            }
        },
        'R' => {
            for i in 1..=dist {
                coords.push((Coordinate { x: curr.x + i, y: curr.y }, *len));
                *len += 1;
            }
        },
        _ => panic!("Unexpected match"),
    }

    coords
}

fn get_wire(line: &str) -> HashMap<Coordinate, i32> {
    // Split the line and get (char, usize)
    let ops: Vec<(char, i32)> = line.split(',')
        .map(|x| {
            let mut chars = x.chars();
            (chars.next().unwrap(), chars.collect::<String>().parse().unwrap())
        })
        .collect();

    let mut wire: HashMap<Coordinate, i32> = HashMap::new();
    let mut curr: Coordinate = Coordinate { x: 0, y: 0 };
    let mut dist: i32 = 1;

    for op in ops {
        let coords: Vec<(Coordinate, i32)> = simulate(curr, op, &mut dist);

        for (c, d) in &coords {
            if !wire.contains_key(c) {
                wire.insert(*c, *d);
            }
        }

        curr = coords[coords.len() - 1].0;
    }

    wire
}

fn get_wires(filename: &str) -> (HashMap<Coordinate, i32>, HashMap<Coordinate, i32>) {
    // Read the file to a Vec<String>
    let file_lines: Vec<String> = fs::read_to_string(filename)
        .expect("Failed to read the problem input")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x))
        .collect();

    (get_wire(&file_lines[0]), get_wire(&file_lines[1]))
}

fn get_min_value(a: HashMap<Coordinate, i32>, b: HashMap<Coordinate, i32>) -> i32 {
    let mut min_value: i32 = std::i32::MAX;

    for k in a.keys() {
        if b.contains_key(k) {
            if k.x == 0 && k.y == 0 {
                continue;
            }

            let val: i32 = a.get(k).unwrap() + b.get(k).unwrap();

            if val < min_value {
                min_value = val;
                dbg!(min_value);
            }
        }
    }

    min_value
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please supply a filename");
    }

    let (a, b) = get_wires(&args[1]);
    let min: i32 = get_min_value(a, b);
    dbg!(min);
}
