use std::fs;

#[derive(Debug, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn parse_dig_plan(contents: &String) -> Result<Vec<(Direction, usize, u32)>, String> {
  contents
    .lines()
    .map(|line| {
      let mut parts = line.split_whitespace();

      let direction = match parts.next() {
        Some(direction) => direction,
        None => return Err(format!("unable to parse direction in line \"{line}\"")),
      };
      let direction = match direction.chars().next() {
        Some(direction) => {
          match direction {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return Err(format!("unable to parse direction in line \"{line}\": unexpected direction \"{direction}\"")),
          }
        },
        None => return Err(format!("unable to parse direction in line \"{line}\"")),
      };

      let steps = match parts.next() {
        Some(steps) => steps,
        None => return Err(format!("unable to parse steps in line \"{line}\"")),
      };
      let steps = match steps.parse::<usize>() {
        Ok(steps) => steps,
        Err(error) => return Err(format!("unable to parse steps in line \"{line}\": {error}")),
      };

      let color = match parts.next() {
        Some(color) => color,
        None => return Err(format!("unable to parse color in line \"{line}\"")),
      };
      if color.len() != "(#xxxxxx)".len() {
        return Err(format!("unable to parse color in line \"{line}\""));
      }

      let red = match u32::from_str_radix(&color[2..4], 16) {
        Ok(red) => red,
        Err(error) => return Err(format!("unable to parse color in line \"{line}\": {error}")),
      };
      let blue = match u32::from_str_radix(&color[4..6], 16) {
        Ok(blue) => blue,
        Err(error) => return Err(format!("unable to parse color in line \"{line}\": {error}")),
      };
      let green = match u32::from_str_radix(&color[6..8], 16) {
        Ok(green) => green,
        Err(error) => return Err(format!("unable to parse color in line \"{line}\": {error}")),
      };

      let color = red << 16 | blue << 8 | green;

      Ok((direction, steps, color))
    })
    .into_iter()
    .collect()
}

struct NegativeOffset {
  up: isize,
  left: isize,
}

fn gather_extents(dig_plan: &Vec<(Direction, usize, u32)>) -> NegativeOffset {
  let mut up = 0;
  let mut left = 0;

  let mut current_position = (0isize, 0isize);

  for (direction, steps, _) in dig_plan {
    match direction {
      Direction::Up => {
        current_position.0 = current_position.0 - *steps as isize;
        up = up.min(current_position.0);
      },
      Direction::Left => {
        current_position.1 = current_position.1 - *steps as isize;
        left = left.min(current_position.1);
      },
      _ => {},
    }
  }

  NegativeOffset{ up, left }
}

fn turn_weight(previous_direction: &Option<&Direction>, direction: &Direction) -> isize {
  if let &Some(previous_direction) = previous_direction {
    if previous_direction == direction {
      return 0;
    }

    match previous_direction {
      Direction::Up => {
        if direction == &Direction::Left {
          return -1;
        } else if direction == &Direction::Right {
          return 1;
        } else {
          return 0;
        }
      },
      Direction::Down => {
        if direction == &Direction::Left {
          return 1;
        } else if direction == &Direction::Right {
          return -1;
        } else {
          return 0;
        }
      },
      Direction::Left => {
        if direction == &Direction::Up {
          return 1;
        } else if direction == &Direction::Down {
          return -1;
        } else {
          return 0;
        }
      },
      Direction::Right => {
        if direction == &Direction::Up {
          return -1;
        } else if direction == &Direction::Down {
          return 1;
        } else {
          return 0;
        }
      },
    }
  } else {
    return 0;
  }
}

fn count_turns(dig_plan: &Vec<(Direction, usize, u32)>) -> isize {
  let mut previous_direction = None;
  let mut turns = 0;

  for (direction, _, _) in dig_plan {
    turns = turns + turn_weight(&previous_direction, direction);
    previous_direction = Some(direction);
  }

  turns
}

fn fix_dig_plan(mut dig_plan: Vec<(Direction, usize, u32)>) -> Result<Vec<(Direction, usize, u32)>, String> {
  for i in 0..dig_plan.len() {
    let direction = match dig_plan[i].2 & 0xf {
      0 => Direction::Right,
      1 => Direction::Down,
      2 => Direction::Left,
      3 => Direction::Up,
      _ => return Err(format!("unable to extract direction from color {:#07x}", dig_plan[i].2)),
    };

    let steps = (dig_plan[i].2 >> 4) as usize;

    dig_plan[i].0 = direction;
    dig_plan[i].1 = steps;
  }

  Ok(dig_plan)
}

fn calculate_area_between_ceils_and_floors(dig_plan: &Vec<(Direction, usize, u32)>, negative_offset: &NegativeOffset, turns: isize) -> usize {
  let mut positive_area = 0;
  let mut negative_area = 0;

  let mut row = negative_offset.up.abs_diff(0);
  let mut col = negative_offset.left.abs_diff(0);

  if turns >= 0 {
    for index in 0..dig_plan.len() {
      let prev_index = (dig_plan.len() - 1 + index) % dig_plan.len();
      let next_index = (dig_plan.len() + 1 + index) % dig_plan.len();

      let prev_direction = &dig_plan[prev_index].0;
      let direction = &dig_plan[index].0;
      let next_direction = &dig_plan[next_index].0;

      let steps = dig_plan[index].1;

      match direction {
        Direction::Up => { row = row - steps; },
        Direction::Down => { row = row + steps; },
        Direction::Left => {
          let mut width = steps;
          let height = row + 1;

          if prev_direction == &Direction::Down {
            width = width + 1;
          }

          if next_direction == &Direction::Down {
            width = width - 1;
          }

          positive_area = positive_area + width * height;

          col = col - steps;
        },
        Direction::Right => {
          let mut width = steps;
          let height = row;

          if prev_direction == &Direction::Up {
            width = width + 1;
          }

          if next_direction == &Direction::Up {
            width = width - 1;
          }

          negative_area = negative_area + width * height;

          col = col + steps;
        },
      }
    }
  } else {
    for index in 0..dig_plan.len() {
      let prev_index = (dig_plan.len() - 1 + index) % dig_plan.len();
      let next_index = (dig_plan.len() + 1 + index) % dig_plan.len();

      let prev_direction = &dig_plan[prev_index].0;
      let direction = &dig_plan[index].0;
      let next_direction = &dig_plan[next_index].0;

      let steps = dig_plan[index].1;

      match direction {
        Direction::Up => { row = row - steps; },
        Direction::Down => { row = row + steps; },
        Direction::Left => {
          let mut width = steps;
          let height = row;

          if prev_direction == &Direction::Up {
            width = width + 1;
          }

          if next_direction == &Direction::Up {
            width = width - 1;
          }

          negative_area = negative_area + width * height;

          col = col - steps;
        },
        Direction::Right => {
          let mut width = steps;
          let height = row + 1;

          if prev_direction == &Direction::Down {
            width = width + 1;
          }

          if next_direction == &Direction::Down {
            width = width - 1;
          }

          positive_area = positive_area + width * height;

          col = col + steps;
        },
      }
    }
  }

  positive_area - negative_area
}

fn part1(contents: &String) -> Result<usize, String> {
  let dig_plan = match parse_dig_plan(contents) {
    Ok(dig_plan) => dig_plan,
    Err(error) => return Err(error),
  };

  let extents = gather_extents(&dig_plan);

  let turns = count_turns(&dig_plan);

  let area = calculate_area_between_ceils_and_floors(&dig_plan, &extents, turns);

  Ok(area)
}

fn part2(contents: &String) -> Result<usize, String> {
  let dig_plan = match parse_dig_plan(contents) {
    Ok(dig_plan) => dig_plan,
    Err(error) => return Err(error),
  };

  let dig_plan = match fix_dig_plan(dig_plan) {
    Ok(dig_plan) => dig_plan,
    Err(error) => return Err(error),
  };

  let extents = gather_extents(&dig_plan);

  let turns = count_turns(&dig_plan);

  let area = calculate_area_between_ceils_and_floors(&dig_plan, &extents, turns);

  Ok(area)
}

fn main() {
  let file_contents = fs::read_to_string("input.txt");

  let contents = match file_contents {
    Ok(contents) => contents,
    Err(error) => panic!("file not found: {}", error),
  };

  match part1(&contents) {
    Ok(result) => println!("part1: {}", result),
    Err(error) => println!("part1: {}", error),
  }

  match part2(&contents) {
    Ok(result) => println!("part2: {}", result),
    Err(error) => println!("part2: {}", error),
  }
}