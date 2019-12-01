use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn fuel_for_mass(mass: i32) -> i32 {
    return ((mass as f32 / 3 as f32).floor() as i32) - 2;
}

fn fuel_with_weight_for_mass(mass: i32) -> i32 {
    let required_fuel = ((mass as f32 / 3 as f32).floor() as i32) - 2;
    if required_fuel <= 0 {
        return 0;
    }

    return required_fuel + fuel_with_weight_for_mass(required_fuel);
}

fn calculate_total_fuel_for_masses(masses: &Vec<i32>) -> i32 {
    return masses.into_iter().map(|m| fuel_for_mass(*m)).sum();
}

fn calculate_total_fuel_with_weight_for_masses(masses: &Vec<i32>) -> i32 {
    return masses
        .into_iter()
        .map(|m| fuel_with_weight_for_mass(*m))
        .sum();
}

fn masses_from_file(file: &str) -> Vec<i32> {
    let path = Path::new(&file);
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

    let masses = content.split("\n");

    return masses
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let masses = masses_from_file(file);

    let total_fuel = calculate_total_fuel_for_masses(&masses);
    let total_fuel_with_weight = calculate_total_fuel_with_weight_for_masses(&masses);

    println!("Total fuel is: {}", total_fuel);
    println!("Total fuel (weighted) is: {}", total_fuel_with_weight);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(12), 2);
        assert_eq!(fuel_for_mass(14), 2);
        assert_eq!(fuel_for_mass(1969), 654);
        assert_eq!(fuel_for_mass(100756), 33_583);
    }

    #[test]
    fn test_fuel_with_weight_for_mass() {
        assert_eq!(fuel_with_weight_for_mass(14), 2);
        assert_eq!(fuel_with_weight_for_mass(1969), 966);
        assert_eq!(fuel_with_weight_for_mass(100756), 50346);
    }
}
