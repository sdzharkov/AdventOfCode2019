use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Self {
      Point { x, y }
  }
}

#[derive(Copy, Clone, Debug)]
struct Line(Point, Point);
impl Line {
  pub fn intersect(self, other: Self) -> Option<Point> {
    println!("{:?} -> {:?}", self, other);
    let a1 = self.1.y - self.0.y;
    let b1 = self.0.x - self.1.x;
    let c1 = a1 * self.0.x + b1 * self.0.y;

    let a2 = other.1.y - other.0.y;
    let b2 = other.0.x - other.1.x;
    let c2 = a2 * other.0.x + b2 * other.0.y;

    let delta = a1 * b2 - a2 * b1;

    println!("Delta: {}", delta);
    if delta == 0 {
      return None;
    }

    Some(Point {
      x: (b2 * c1 - b1 * c2) / delta,
      y: (a1 * c2 - a2 * c1) / delta,
    })
  }
}

#[derive(Debug)]
struct Path {
  coordinates: HashSet<Point>,
  lines: Vec<Line>,
}

impl Path {
  pub fn new(points: &Vec<&str>) -> Path {
    let mut coordinates = HashSet::new();
    let mut lines = Vec::new();
    let mut prev_point = Point { x: 0, y: 0 };
    let mut next_point: Point;
    let mut number: i32;

    for point in points {
      number = point[1..].parse::<i32>().unwrap();

      next_point = match point.chars().next() {
        Some('L') => Some(Point {
          x: prev_point.x - number,
          y: prev_point.y,
        }),
        Some('R') => Some(Point {
          x: prev_point.x + number,
          y: prev_point.y,
        }),
        Some('U') => Some(Point {
          x: prev_point.x,
          y: prev_point.y + number,
        }),
        Some('D') => Some(Point {
          x: prev_point.x,
          y: prev_point.y - number,
        }),
        _ => None,
      }
      .unwrap();

      coordinates.insert(next_point);
      lines.push(Line(prev_point, next_point));
      // {
      //   println!("Prev: {:?}, Point: {}, Next: {:?}", prev_point, point, next_point)
      // }
      prev_point = next_point;
    }

    Path {
      coordinates: coordinates,
      lines: lines,
    }
  }
}

fn parse<'a>(contents: &'a String) -> (Vec<&str>, Vec<&str>) {
  let res: Vec<&str> = contents.split('\n').collect();

  let first: Vec<&str> = res[0].split(',').collect();
  let second: Vec<&str> = res[1].split(',').collect();

  return (first, second);
}

pub fn run() {
  let contents = fs::read_to_string("input/day3_ex_1.txt").unwrap();
  let (path_1, path_2) = parse(&contents);

  let first_path = Path::new(&path_1);
  let second_path = Path::new(&path_2);

  println!("Path 1: {:?}\n", first_path.lines);
  println!("Path 2: {:?}\n", second_path.lines);

  // let intersection = first_path.coordinates.intersection(&second_path.coordinates);
  // println!("{:?}", intersection);

  // let mut intersections = Vec::new();
  // for path in first_path.lines {
  //   for s_path in &second_path.lines {
  //     match path.intersect(*s_path) {
  //       Some(x) => {
  //         intersections.push(x);
  //       }
  //       None => {}
  //     }
  //   }
  // }

  // println!("{:?}", intersections);

  let l1 = Line(Point::new(1, 1), Point::new(1, 8));
  let l2 = Line(Point::new(3, 3), Point::new(6, 6));
  println!("{:?}", l1.intersect(l2));
}
