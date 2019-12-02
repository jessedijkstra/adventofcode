use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const ADD_OP_CODE: i32 = 1;
const MULTIPLY_OP_CODE: i32 = 2;
const HALT_OP_CODE: i32 = 99;

fn get_positions(int_code: &[i32], position: usize) -> (usize, usize, usize) {
    return (
        int_code[position + 1] as usize,
        int_code[position + 2] as usize,
        int_code[position + 3] as usize,
    );
}

fn run(mut int_code: Vec<i32>, position: usize) -> Vec<i32> {
    match int_code.get(position) {
        Some(&ADD_OP_CODE) => {
            let (first_position, second_position, output_position) =
                get_positions(int_code.as_slice(), position);

            let first_value = int_code[first_position];
            let second_value = int_code[second_position];

            std::mem::replace(&mut int_code[output_position], first_value + second_value);

            return run(int_code, position + 4);
        }
        Some(&MULTIPLY_OP_CODE) => {
            let (first_position, second_position, output_position) =
                get_positions(int_code.as_slice(), position);

            let first_value = int_code[first_position];
            let second_value = int_code[second_position];

            std::mem::replace(&mut int_code[output_position], first_value * second_value);

            return run(int_code, position + 4);
        }
        Some(&HALT_OP_CODE) => {
            return int_code;
        }
        Some(value) => panic!("Invalid opcode {}", value),
        None => int_code,
    }
}

fn read_file(fileName: &str) -> String {
    let path = Path::new(&fileName);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut string = String::new();

    let content = match file.read_to_string(&mut string) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => string,
    };

    return content;
}

fn csv_to_vector(csv: String) -> Vec<i32> {
    return csv
        .split(",")
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let mut int_code = csv_to_vector(read_file(file));

    std::mem::replace(&mut int_code[1], 12);
    std::mem::replace(&mut int_code[2], 2);

    println!("{:?}", run(int_code, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode() {
        assert_eq!(run(vec![1, 0, 0, 0, 99], 0), vec![2, 0, 0, 0, 99]);

        assert_eq!(run(vec![2, 3, 0, 3, 99], 0), vec![2, 3, 0, 6, 99]);

        assert_eq!(run(vec![2, 4, 4, 5, 99, 0], 0), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], 0),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
