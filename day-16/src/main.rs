use std::fs;

fn parse_grid(contents: &String) -> Vec<Vec<char>> {
  contents
    .lines()
    .map(|line| line.chars().collect())
    .collect()
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

struct Beam {
  row: usize,
  col: usize,
  direction: Direction,
}

enum NextBeam {
  Singular(Direction),
  Split(Direction, Direction),
}

fn next_beam(grid: &Vec<Vec<char>>, position: &(usize, usize), direction: &Direction) -> NextBeam {
  match direction {
    Direction::Up => {
      match grid[position.0][position.1] {
        '-' => NextBeam::Split(Direction::Left, Direction::Right),
        '/' => NextBeam::Singular(Direction::Right),
        '\\' => NextBeam::Singular(Direction::Left),
        _ => NextBeam::Singular(Direction::Up),
      }
    },
    Direction::Down => {
      match grid[position.0][position.1] {
        '-' => NextBeam::Split(Direction::Left, Direction::Right),
        '/' => NextBeam::Singular(Direction::Left),
        '\\' => NextBeam::Singular(Direction::Right),
        _ => NextBeam::Singular(Direction::Down),
      }
    },
    Direction::Left => {
      match grid[position.0][position.1] {
        '|' => NextBeam::Split(Direction::Up, Direction::Down),
        '/' => NextBeam::Singular(Direction::Down),
        '\\' => NextBeam::Singular(Direction::Up),
        _ => NextBeam::Singular(Direction::Left),
      }
    },
    Direction::Right => {
      match grid[position.0][position.1] {
        '|' => NextBeam::Split(Direction::Up, Direction::Down),
        '/' => NextBeam::Singular(Direction::Up),
        '\\' => NextBeam::Singular(Direction::Down),
        _ => NextBeam::Singular(Direction::Right),
      }
    },
  }
}

fn make_empty_energized_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<u8>> {
  grid
    .iter()
    .map(|row| vec![0; row.len()])
    .collect()
}

#[derive(Debug)]
struct TileJump {
  up: usize,
  down: usize,
  left: usize,
  right: usize,
}

fn make_jump_table(grid: &Vec<Vec<char>>) -> Vec<Vec<TileJump>> {
  grid
    .iter()
    .enumerate()
    .map(|(row_index, row)| {
      (0..row.len())
        .map(|col_index| {
          let up = (0..row_index)
            .rev()
            .find(|&index| grid[index][col_index] != '.' && grid[index][col_index] != '|')
            .unwrap_or(0);

          let down = ((row_index + 1)..grid.len())
            .find(|&index| grid[index][col_index] != '.' && grid[index][col_index] != '|')
            .unwrap_or(grid.len() - 1);

          let left = (0..col_index)
            .rev()
            .find(|&index| grid[row_index][index] != '.' && grid[row_index][index] != '-')
            .unwrap_or(0);

          let right = ((col_index + 1)..row.len())
            .find(|&index| grid[row_index][index] != '.' && grid[row_index][index] != '-')
            .unwrap_or(row.len() - 1);

          TileJump{ up, down, left, right }
        })
        .collect()
    })
    .collect()
}

fn move_throught_beam(energized_grid: &mut Vec<Vec<u8>>, jump_table: &Vec<Vec<TileJump>>, beam: &Beam) -> Option<Beam> {
  let jump_table = &jump_table[beam.row][beam.col];

  match beam.direction {
    Direction::Up => {
      for i in (jump_table.up..beam.row).rev() {
        if (energized_grid[i][beam.col] & 1) == 1 {
          return None;
        }

        energized_grid[i][beam.col] = energized_grid[i][beam.col] | 1;
      }

      if beam.row != jump_table.up {
        return Some(Beam{ row: jump_table.up, col: beam.col, direction: Direction::Up });
      }

      return None;
    },
    Direction::Down => {
      for i in (beam.row + 1)..=jump_table.down {
        if (energized_grid[i][beam.col] & 2) == 2 {
          return None;
        }

        energized_grid[i][beam.col] = energized_grid[i][beam.col] | 2;
      }

      if beam.row != jump_table.down {
        return Some(Beam{ row: jump_table.down, col: beam.col, direction: Direction::Down });
      }

      return None;
    },
    Direction::Left => {
      for i in (jump_table.left..beam.col).rev() {
        if (energized_grid[beam.row][i] & 4) == 4 {
          return None;
        }

        energized_grid[beam.row][i] = energized_grid[beam.row][i] | 4;
      }

      if beam.col != jump_table.left {
        return Some(Beam{ row: beam.row, col: jump_table.left, direction: Direction::Left });
      }

      return None;
    },
    Direction::Right => {
      for i in (beam.col + 1)..=jump_table.right {
        if (energized_grid[beam.row][i] & 8) == 8 {
          return None;
        }

        energized_grid[beam.row][i] = energized_grid[beam.row][i] | 8;
      }

      if beam.col != jump_table.right {
        return Some(Beam{ row: beam.row, col: jump_table.right, direction: Direction::Right });
      }

      return None;
    },
  }
}

fn collect_energized_tiles_count(energized_grid: &Vec<Vec<u8>>) -> usize {
  energized_grid
    .iter()
    .map(|row| {
      row
        .iter()
        .map(|&v| if v == 0 { 0 } else { 1 })
        .sum::<usize>()
    })
    .sum::<usize>()
}

fn part1(contents: &String) -> Result<usize, String> {
  let grid: Vec<Vec<char>> = parse_grid(contents);
  let jump_table = make_jump_table(&grid);
  let mut energized_grid = make_empty_energized_grid(&grid);
  let mut beams = vec![];

  {
    let beam = Beam{ row: 0, col: 0, direction: Direction::Right };

    match next_beam(&grid, &(beam.row, beam.col), &beam.direction) {
      NextBeam::Split(direction0, direction1) => {
        beams.push(Beam{ row: beam.row, col: beam.col, direction: direction0 });
        beams.push(Beam{ row: beam.row, col: beam.col, direction: direction1 });
      },
      NextBeam::Singular(direction) => {
        beams.push(Beam{ row: beam.row, col: beam.col, direction });
      },
    }

    match beam.direction {
      Direction::Up => { energized_grid[beam.row][beam.col] = energized_grid[beam.row][beam.col] | 1; },
      Direction::Down => { energized_grid[beam.row][beam.col] = energized_grid[beam.row][beam.col] | 2; },
      Direction::Left => { energized_grid[beam.row][beam.col] = energized_grid[beam.row][beam.col] | 4; },
      Direction::Right => { energized_grid[beam.row][beam.col] = energized_grid[beam.row][beam.col] | 8; },
    }
  }

  loop {
    if beams.is_empty() {
      break;
    }

    let beam = beams.swap_remove(0);

    if let Some(beam) = move_throught_beam(&mut energized_grid, &jump_table, &beam) {

      match next_beam(&grid, &(beam.row, beam.col), &beam.direction) {
        NextBeam::Split(direction0, direction1) => {
          beams.push(Beam{ row: beam.row, col: beam.col, direction: direction0 });
          beams.push(Beam{ row: beam.row, col: beam.col, direction: direction1 });
        },
        NextBeam::Singular(direction) => {
          beams.push(Beam{ row: beam.row, col: beam.col, direction });
        },
      }
    }
  }

  Ok(collect_energized_tiles_count(&energized_grid))
}

fn part2(contents: &String) -> Result<usize, String> {
  let grid: Vec<Vec<char>> = parse_grid(contents);
  let jump_table = make_jump_table(&grid);

  let iter0 = (0..grid.len()).map(|row_index| Beam{ row: row_index, col: 0, direction: Direction::Right });
  let iter1 = (0..grid.len()).map(|row_index| Beam{ row: row_index, col: grid[0].len() - 1, direction: Direction::Left });
  let iter2 = (0..grid[0].len()).map(|col_index| Beam{ row: 0, col: col_index, direction: Direction::Down });
  let iter3 = (0..grid[0].len()).map(|col_index| Beam{ row: grid.len() - 1, col: col_index, direction: Direction::Up });

  let max_energized_tiles_count = iter0.chain(iter1).chain(iter2).chain(iter3)
    .map(|beam| {
      let mut energized_grid = make_empty_energized_grid(&grid);
      let mut beams = vec![];

      match next_beam(&grid, &(beam.row, beam.col), &beam.direction) {
        NextBeam::Split(direction0, direction1) => {
          beams.push(Beam{ row: beam.row, col: beam.col, direction: direction0 });
          beams.push(Beam{ row: beam.row, col: beam.col, direction: direction1 });
        },
        NextBeam::Singular(direction) => {
          beams.push(Beam{ row: beam.row, col: beam.col, direction });
        },
      }

      match beam.direction {
        Direction::Up => { energized_grid[beam.row][beam.col] = energized_grid[beam.row][beam.col] | 1; },
        Direction::Down => { energized_grid[beam.row][beam.col] = energized_grid[beam.row][beam.col] | 2; },
        Direction::Left => { energized_grid[beam.row][beam.col] = energized_grid[beam.row][beam.col] | 4; },
        Direction::Right => { energized_grid[beam.row][beam.col] = energized_grid[beam.row][beam.col] | 8; },
      }

      loop {
        if beams.is_empty() {
          break;
        }

        let beam = beams.swap_remove(0);

        if let Some(beam) = move_throught_beam(&mut energized_grid, &jump_table, &beam) {
          match next_beam(&grid, &(beam.row, beam.col), &beam.direction) {
            NextBeam::Split(direction0, direction1) => {
              beams.push(Beam{ row: beam.row, col: beam.col, direction: direction0 });
              beams.push(Beam{ row: beam.row, col: beam.col, direction: direction1 });
            },
            NextBeam::Singular(direction) => {
              beams.push(Beam{ row: beam.row, col: beam.col, direction });
            },
          }
        }
      }

      collect_energized_tiles_count(&energized_grid)
    })
    .max()
    .unwrap_or(0);

  Ok(max_energized_tiles_count)
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