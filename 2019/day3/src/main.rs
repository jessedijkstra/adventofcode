use std::cmp::min;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path as FilePath;

type X = i32;
type Y = i32;
type Moves = i32;
type Direction = String;
type Instruction = (Direction, Moves);
type Point = (X, Y);
type PointWithValue = (Point, i32);
type Path = Vec<Instruction>;
type Board = Vec<PointWithValue>;

fn read_file(file_name: &str) -> String {
    let path = FilePath::new(&file_name);
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

fn string_to_path(string: String) -> Path {
    return string
        .split(',')
        .map(|s| {
            let (direction, moves) = s.split_at(1);

            return (direction.to_string(), moves.parse::<i32>().unwrap());
        })
        .collect();
}

fn input_to_paths(csv: String) -> Vec<Vec<(String, i32)>> {
    return csv
        .split("\n")
        .into_iter()
        .map(|s| string_to_path(s.to_string()))
        .collect();
}

fn increment_position(board: &mut Board, position: usize) {
    let ((x, y), value) = board[position].clone();

    std::mem::replace(&mut board[position], ((x, y), value + 1));
}

fn increment_point(board: &mut Board, target_x: i32, target_y: i32) {
    match board
        .into_iter()
        .position(|((x, y), _value)| *x == target_x && *y == target_y)
    {
        None => board.push(((target_x.clone(), target_y.clone()), 1)),
        Some(position) => increment_position(board, position),
    }
}

fn draw_horizontal_line(board: &mut Board, point: Point, to_x: X) {
    let (x, y) = point;

    for target_x in (x + 1)..to_x {
        increment_point(board, target_x, y);
    }
}

fn draw_vertical_line(board: &mut Board, point: Point, to_y: Y) {
    let (x, y) = point;

    for target_y in (y + 1)..to_y {
        increment_point(board, x, target_y);
    }
}

fn draw_path(path: Path, board: &mut Board) -> &mut Board {
    let mut current_y = 0;
    let mut current_x = 0;

    for instruction in path {
        let (direction, moves) = instruction;

        match direction.as_ref() {
            "U" => {
                let target_y = current_y + moves;

                draw_vertical_line(board, (current_x, current_y), target_y);

                current_y = target_y;
            }
            "D" => {
                let target_y = current_y - moves;

                draw_vertical_line(board, (current_x, current_y), target_y);

                current_y = target_y;
            }
            "R" => {
                let target_y = current_y + moves;

                draw_horizontal_line(board, (current_x, current_y), target_y);

                current_y = target_y;
            }
            "L" => {
                let target_x = current_x - moves;

                draw_horizontal_line(board, (current_x, current_x), target_x);

                current_x = target_x;
            }
            value => panic!("unknown direction {}", value),
        }

        increment_point(board, current_x, current_y);
    }

    return board;
}

fn distance(from: Point, to: Point) -> i32 {
    let mut distance = 0;

    let (from_x, from_y) = from;
    let (to_x, to_y) = to;
    distance = distance + (to_x - from_x).abs();
    distance = distance + (to_y - from_y).abs();
    return distance;
}

fn distance_to_nearest_intersection(board: Board) -> i32 {
    return board
        .into_iter()
        .filter(|point_with_value| {
            let ((_, _), value) = point_with_value;

            return *value == 2;
        })
        .map(|point_with_value| {
            let (point, _value) = point_with_value;

            return point;
        })
        .fold(0, |acc, point| {
            let distance = distance((0, 0), point);

            if acc == 0 {
                return distance;
            }

            return min(acc, distance);
        });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let paths = input_to_paths(read_file(file));

    let first: Path = paths[0].clone();
    let second: Path = paths[1].clone();

    let mut board: Board = vec![];

    draw_path(first, &mut board);
    draw_path(second, &mut board);

    println!(
        "Distance to nearest intersection: {:?}",
        distance_to_nearest_intersection(board)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_intersection() {
        let line = string_to_path("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string());
        let line2 = string_to_path("U62,R66,U55,R34,D71,R55,D58,R83".to_string());

        let mut board = vec![];

        draw_path(line, &mut board);
        draw_path(line2, &mut board);

        assert_eq!(distance_to_nearest_intersection(board), 159);
    }
}
