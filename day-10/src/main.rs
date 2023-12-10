use std::fs;

struct PipeMap {
  start_position: (usize, usize),
  schema: Vec<Vec<char>>,
}

fn find_start_position(schema: &Vec<Vec<char>>) -> Option<(usize, usize)> {
  for (row_index, row) in schema.iter().enumerate() {
    for (col_index, &symbol) in row.iter().enumerate() {
      if symbol == 'S' {
        return Some((row_index, col_index));
      }
    }
  }

  None
}

fn parse_pipe_map(contents: &String) -> Result<PipeMap, String> {
  let schema = contents
    .lines()
    .map(|line| {
      line
        .chars()
        .collect::<Vec<char>>()
    })
    .collect::<Vec<Vec<char>>>();

  let start_position = match find_start_position(&schema) {
    Some(start_position) => start_position,
    None => return Err("starting position not found".into()),
  };

  Ok(PipeMap{
    start_position,
    schema,
  })
}

#[derive(Debug, Clone)]
enum Direction {
  Up, Down, Left, Right,
}

fn find_start_direction(pipe_map: &PipeMap) -> Result<Direction, String> {
  let position = pipe_map.start_position;

  if position.0 > 0 {
    if pipe_map.schema[position.0 - 1][position.1] == 'F' || pipe_map.schema[position.0 - 1][position.1] == '7' || pipe_map.schema[position.0 - 1][position.1] == '|' {
      return Ok(Direction::Up);
    }
  }

  if position.1 + 1 < pipe_map.schema.len() {
    if pipe_map.schema[position.0 + 1][position.1] == '|' || pipe_map.schema[position.0 + 1][position.1] == 'L' || pipe_map.schema[position.0 + 1][position.1] == 'J' {
      return Ok(Direction::Down);
    }
  }

  if position.1 > 0 {
    if pipe_map.schema[position.0][position.1 - 1] == '-' || pipe_map.schema[position.0][position.1 - 1] == 'F' || pipe_map.schema[position.0][position.1 - 1] == 'L' {
      return Ok(Direction::Left);
    }
  }

  if position.1 + 1 < pipe_map.schema[position.0].len() {
    if pipe_map.schema[position.0][position.1 + 1] == '-' || pipe_map.schema[position.0][position.1 + 1] == '7' || pipe_map.schema[position.0][position.1 + 1] == 'J' {
      return Ok(Direction::Right);
    }
  }

  Err("unable to find a valid path from starting position".into())
}

fn find_loop_length(pipe_map: &PipeMap, start_direction: &Direction) -> Result<usize, String> {
  let mut position = pipe_map.start_position.clone();
  let mut direction = start_direction.clone();

  let mut length = 1;
  loop {
    position = match direction {
      Direction::Up => (position.0 - 1, position.1),
      Direction::Down => (position.0 + 1, position.1),
      Direction::Left => (position.0, position.1 - 1),
      Direction::Right => (position.0, position.1 + 1),
    };

    if pipe_map.schema[position.0][position.1] == 'S' {
      return Ok(length);
    }

    direction = match direction {
      Direction::Up => {
        match pipe_map.schema[position.0][position.1] {
          '|' => Direction::Up,
          '7' => Direction::Left,
          'F' => Direction::Right,
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Down => {
        match pipe_map.schema[position.0][position.1] {
          '|' => Direction::Down,
          'J' => Direction::Left,
          'L' => Direction::Right,
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Left => {
        match pipe_map.schema[position.0][position.1] {
          '-' => Direction::Left,
          'L' => Direction::Up,
          'F' => Direction::Down,
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Right => {
        match pipe_map.schema[position.0][position.1] {
          '-' => Direction::Right,
          'J' => Direction::Up,
          '7' => Direction::Down,
          _ => return Err("unexpected end of the path".into()),
        }
      },
    };

    length = length + 1;
  }
}

#[derive(Debug, Clone, Copy)]
enum Winding {
  CW,
  CCW,
}

fn mark_loop_and_find_winding(pipe_map: &PipeMap, start_direction: &Direction) -> Result<(Vec<Vec<usize>>, Winding), String> {
  let mut markings_map = pipe_map.schema
    .iter()
    .map(|row| vec![0; row.len()])
    .collect::<Vec<Vec<usize>>>();

  let mut position = pipe_map.start_position.clone();
  let mut direction = start_direction.clone();
  let mut winding_counter = 0i64;

  markings_map[position.0][position.1] = 1;

  loop {
    position = match direction {
      Direction::Up => (position.0 - 1, position.1),
      Direction::Down => (position.0 + 1, position.1),
      Direction::Left => (position.0, position.1 - 1),
      Direction::Right => (position.0, position.1 + 1),
    };

    markings_map[position.0][position.1] = 1;

    if pipe_map.schema[position.0][position.1] == 'S' {
      return Ok((markings_map, if winding_counter > 0 { Winding::CW } else { Winding::CCW }));
    }

    (direction, winding_counter) = match direction {
      Direction::Up => {
        match pipe_map.schema[position.0][position.1] {
          '|' => (Direction::Up, winding_counter),
          'F' => (Direction::Right, winding_counter + 1),
          '7' => (Direction::Left, winding_counter - 1),
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Down => {
        match pipe_map.schema[position.0][position.1] {
          '|' => (Direction::Down, winding_counter),
          'J' => (Direction::Left, winding_counter + 1),
          'L' => (Direction::Right, winding_counter - 1),
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Left => {
        match pipe_map.schema[position.0][position.1] {
          '-' => (Direction::Left, winding_counter),
          'L' => (Direction::Up, winding_counter + 1),
          'F' => (Direction::Down, winding_counter - 1),
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Right => {
        match pipe_map.schema[position.0][position.1] {
          '-' => (Direction::Right, winding_counter),
          '7' => (Direction::Down, winding_counter + 1),
          'J' => (Direction::Up, winding_counter - 1),
          _ => return Err("unexpected end of the path".into()),
        }
      },
    };
  }
}

fn flood_fill_markings_map(position: (usize, usize), offset: (isize, isize), markings_map: &mut Vec<Vec<usize>>) -> usize {
  let position = ((position.0 as isize) + offset.0, (position.1 as isize) + offset.1);
  if position.0 < 0 || position.1 < 0 {
    return 0;
  }

  let position = (position.0 as usize, position.1 as usize);
  if position.0 >= markings_map.len() || position.1 >= markings_map[position.0].len() {
    return 0;
  }

  let mut counter = 0;
  let mut stack = vec![position];

  while let Some(position) = stack.pop() {
    if markings_map[position.0][position.1] == 0 {
      counter = counter + 1;
      markings_map[position.0][position.1] = 2;

      if position.0 > 0 {
        if markings_map[position.0 - 1][position.1] == 0 {
          stack.push((position.0 - 1, position.1));
        }
      }

      if position.0 + 1 < markings_map.len() {
        if markings_map[position.0 + 1][position.1] == 0 {
          stack.push((position.0 + 1, position.1));
        }
      }

      if position.1 > 0 {
        if markings_map[position.0][position.1 - 1] == 0 {
          stack.push((position.0, position.1 - 1));
        }
      }

      if position.1 + 1 < markings_map[position.0].len() {
        if markings_map[position.0][position.1 + 1] == 0 {
          stack.push((position.0, position.1 + 1));
        }
      }
    }
  }

  return counter;
}

fn gather_inside_area(pipe_map: &PipeMap, start_direction: &Direction, markings_map: &mut Vec<Vec<usize>>, winding: Winding) -> Result<usize, String> {
  let mut position = pipe_map.start_position.clone();
  let mut direction = start_direction.clone();
  let mut area = 0;

  loop /* for _ in 0..7 */ {
    position = match direction {
      Direction::Up => (position.0 - 1, position.1),
      Direction::Down => (position.0 + 1, position.1),
      Direction::Left => (position.0, position.1 - 1),
      Direction::Right => (position.0, position.1 + 1),
    };

    let symbol = pipe_map.schema[position.0][position.1];

    if symbol == 'S' {
      return Ok(area);
    }

    (direction, area) = match direction {
      Direction::Up => {
        match symbol {
          '|' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, ( 0,  1), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, ( 0, -1), markings_map);
              },
            }

            (Direction::Up, area)
          },
          'F' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, ( 1,  1), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, (-1,  0), markings_map);
                area = area + flood_fill_markings_map(position, (-1, -1), markings_map);
                area = area + flood_fill_markings_map(position, ( 0, -1), markings_map);
              },
            }

            (Direction::Right, area)
          },
          '7' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, (-1,  0), markings_map);
                area = area + flood_fill_markings_map(position, (-1,  1), markings_map);
                area = area + flood_fill_markings_map(position, ( 0,  1), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, ( 1, -1), markings_map);
              },
            }

            (Direction::Left, area)
          },
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Down => {
        match symbol {
          '|' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, ( 0, -1), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, ( 0,  1), markings_map);
              },
            }

            (Direction::Down, area)
          },
          'J' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, (-1, -1), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, ( 0,  1), markings_map);
                area = area + flood_fill_markings_map(position, ( 1,  1), markings_map);
                area = area + flood_fill_markings_map(position, ( 1,  0), markings_map);
              },
            }

            (Direction::Left, area)
          },
          'L' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, ( 0, -1), markings_map);
                area = area + flood_fill_markings_map(position, ( 1, -1), markings_map);
                area = area + flood_fill_markings_map(position, ( 1,  0), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, (-1,  1), markings_map);
              },
            }

            (Direction::Right, area)
          },
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Left => {
        match symbol {
          '-' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, (-1,  0), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, ( 1,  0), markings_map);
              },
            }

            (Direction::Left, area)
          },
          'L' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, (-1,  1), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, ( 0, -1), markings_map);
                area = area + flood_fill_markings_map(position, ( 1, -1), markings_map);
                area = area + flood_fill_markings_map(position, ( 1,  0), markings_map);
              },
            }

            (Direction::Up, area)
          },
          'F' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, ( 0, -1), markings_map);
                area = area + flood_fill_markings_map(position, (-1, -1), markings_map);
                area = area + flood_fill_markings_map(position, (-1,  0), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, ( 1,  1), markings_map);
              },
            }

            (Direction::Down, area)
          },
          _ => return Err("unexpected end of the path".into()),
        }
      },
      Direction::Right => {
        match symbol {
          '-' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, ( 1,  0), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, (-1,  0), markings_map);
              },
            }

            (Direction::Right, area)
          },
          '7' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, ( 1, -1), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, ( 0,  1), markings_map);
                area = area + flood_fill_markings_map(position, (-1,  1), markings_map);
                area = area + flood_fill_markings_map(position, (-1,  0), markings_map);
              },
            }

            (Direction::Down, area)
          },
          'J' => {
            let mut area = area;

            match winding {
              Winding::CW => {
                area = area + flood_fill_markings_map(position, ( 0,  1), markings_map);
                area = area + flood_fill_markings_map(position, ( 1,  1), markings_map);
                area = area + flood_fill_markings_map(position, ( 1,  0), markings_map);
              },
              Winding::CCW => {
                area = area + flood_fill_markings_map(position, (-1, -1), markings_map);
              },
            }

            (Direction::Up, area)
          },
          _ => return Err("unexpected end of the path".into()),
        }
      },
    };
  }
}

fn part1(contents: &String) -> Result<usize, String> {
  let pipe_map = match parse_pipe_map(&contents) {
    Ok(pipe_map) => pipe_map,
    Err(error) => return Err(error),
  };

  let start_direction = match find_start_direction(&pipe_map) {
    Ok(direction) => direction,
    Err(error) => return Err(error),
  };

  find_loop_length(&pipe_map, &start_direction)
    .map(|length| length / 2)
}

fn part2(contents: &String) -> Result<usize, String> {
  let pipe_map = match parse_pipe_map(&contents) {
    Ok(pipe_map) => pipe_map,
    Err(error) => return Err(error),
  };

  let start_direction = match find_start_direction(&pipe_map) {
    Ok(direction) => direction,
    Err(error) => return Err(error),
  };

  let (mut markings_map, winding) = match mark_loop_and_find_winding(&pipe_map, &start_direction) {
    Ok((markings_map, winding)) => (markings_map, winding),
    Err(error) => return Err(error),
  };

  gather_inside_area(&pipe_map, &start_direction, &mut markings_map, winding)
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