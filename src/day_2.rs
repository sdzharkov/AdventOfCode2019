
fn solve(memory: &mut Vec<usize>) {
  let mut instruction_ptr = 0;
  let mut first: usize;
  let mut second: usize;
  let mut pos: usize;
  loop {
    match memory[instruction_ptr] {
      1 | 2 => {
        first = memory[instruction_ptr + 1];
        second = memory[instruction_ptr + 2];
        pos = memory[instruction_ptr + 3];
        memory[pos] = match memory[instruction_ptr] {
          1 => memory[first] + memory[second],
          2 => memory[first] * memory[second],
          _ => 0
        };
      },
      99 => break,
      _ => println!("Fuck!")
    }
    instruction_ptr += 4;
  }
}

fn binary_search(memory: Vec<usize>, final_num: usize, operation: usize, min: usize, max: usize, switch: bool) -> Option<usize> {
  // println!("Trying min: {}, max: {} with operation: {}", min, max, operation);
  if max >= min {
    let mut new_mem = memory.clone();

    let mid_point = min + (max - min) / 2;

    if switch {
      new_mem[1] = mid_point;
      new_mem[2] = operation;
    } else {
      new_mem[2] = mid_point;
      new_mem[1] = operation;
    }
    new_mem[1] = mid_point;
    new_mem[2] = operation;
    solve(&mut new_mem);

    // println!("Final num: {:?} -> {} on midpoint {}", &new_mem[0..3], final_num, mid_point);
    if new_mem[0] == final_num {
      return Some(100 * new_mem[1] + new_mem[2]);
    }

    if new_mem[0] < final_num {
      return binary_search(memory, final_num, operation, mid_point + 1, max, switch);
    } else {
      return binary_search(memory, final_num, operation, min, mid_point - 1, switch);
    }

  } else {
    None
  }
}

fn solve_part_2(memory: Vec<usize>, number: usize) {
  // This solution wouldn't work if both numbers started as the same number (50, 50)
  for i in 1..99 {
    let res = binary_search(memory.clone(), number, i, 0, 99, true);
    match res {      
      Some(x) => {
        println!("Result: {}", x);
        break;  
      },
      None => continue,
    }
  }
}

pub fn run() {
  let mut memory = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0];

  // solve(&mut memory);

  // println!("{:?}", memory);

  solve_part_2(memory.clone(), 19690720);
}

#[test]
fn case_1() {
  let mut memory = vec![1,0,0,0,99];
  solve(&mut memory);
  assert_eq!(memory, [2,0,0,0,99]);
}

#[test]
fn case_2() {
  let mut memory = vec![2,3,0,3,99];
  solve(&mut memory);
  assert_eq!(memory, [2,3,0,6,99]);
}

#[test]
fn case_3() {
  let mut memory = vec![2,4,4,5,99,0];
  solve(&mut memory);
  assert_eq!(memory, [2,4,4,5,99,9801]);
}
