use std::fs::File;
use std::io::{BufRead, BufReader};


fn generate_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn full_fuel_with_mass(mass: i64) -> i64 {
    if mass <= 0 {
        return 0;
    }
    
    println!("Mass: {}", mass);
    return mass + full_fuel_with_mass(generate_fuel(mass));
}

pub fn run() {
    let file = File::open("src/day_1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut fuel_count: i64 = 0;

    for (_index, line) in reader.lines().enumerate() {
        let mass: i64 = line.unwrap().parse().unwrap(); // Ignore errors.

        let add_fuel = full_fuel_with_mass(generate_fuel(mass));
        fuel_count += add_fuel;
        println!("Total: {} \n", add_fuel);
    }

    println!("Fuel count: {}", fuel_count);
}

#[test]
fn test_full_fuel_with_mass() {
    assert_eq!(full_fuel_with_mass(generate_fuel(100756)), 50346);
}
