use std::fs;



struct Point {
  x: i32,
  y: i32,
}

fn parse() -> (&str, &str) {
  let mut contents = fs::read_to_string("input/day3.txt").unwrap();
  let res: Vec<&str> = contents.split('\n').collect();

  return (res[0], res[1])
}

pub fn run() {
  parse();
}
