use std::fs;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn simulate(curr: Coordinate, op: (char, i32)) -> Vec<Coordinate> {
    let (dir, dist) = op;
    let mut coords: Vec<Coordinate> = Vec::new();

    match dir {
        'U' => {
            for i in 1..=dist {
                coords.push(Coordinate { x: curr.x, y: curr.y + i });
            }
        },
        'D' => {
            for i in 1..=dist {
                coords.push(Coordinate { x: curr.x, y: curr.y - i });
            }
        },
        'L' => {
            for i in 1..=dist {
                coords.push(Coordinate { x: curr.x - i, y: curr.y });
            }
        },
        'R' => {
            for i in 1..=dist {
                coords.push(Coordinate { x: curr.x + i, y: curr.y });
            }
        },
        _ => panic!("Unexpected match"),
    }

    coords
}

fn get_wire(line: &str) -> HashSet<Coordinate> {
    // Split the line and get (char, usize)
    let ops: Vec<(char, i32)> = line.split(',')
        .map(|x| {
            let mut chars = x.chars();
            (chars.next().unwrap(), chars.collect::<String>().parse().unwrap())
        })
        .collect();

    let mut wire: HashSet<Coordinate> = HashSet::new();
    let mut curr: Coordinate = Coordinate { x: 0, y: 0 };

    for op in ops {
        let coords: Vec<Coordinate> = simulate(curr, op);

        for c in &coords {
            wire.insert(*c);
        }

        curr = coords[coords.len() - 1];
    }

    wire
}

fn get_wires(filename: &str) -> (HashSet<Coordinate>, HashSet<Coordinate>) {
    // Read the file to a Vec<String>
    let file_lines: Vec<String> = fs::read_to_string(filename)
        .expect("Failed to read the problem input")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x))
        .collect();

    (get_wire(&file_lines[0]), get_wire(&file_lines[1]))
}

fn main() {
    let (a, b) = get_wires("input.txt");

    let matches: HashSet<&Coordinate> = a.intersection(&b).collect();

    let minima: &Coordinate = matches.iter().min_by_key(|c| c.x.abs() + c.y.abs()).unwrap();

    dbg!(&matches);
    dbg!(&minima);

    println!("Answer is: {}", minima.x.abs() + minima.y.abs());
}
