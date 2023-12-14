use std::{fs, collections::HashMap};

fn parse_platform_map(contents: &String) -> Vec<Vec<char>> {
  contents
    .lines()
    .map(|line| line.chars().collect())
    .collect()
}

fn calculate_total_load(platform_map: &Vec<Vec<char>>) -> usize {
  let row_count = platform_map.len();

  platform_map
    .iter()
    .enumerate()
    .map(|(index, row)| {
      row
        .iter()
        .map(|char| {
          match char {
            'O' => row_count - index,
            _ => 0,
          }
        })
        .sum::<usize>()
    })
    .sum::<usize>()
}

fn tilt_north(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
  let row_count = platform_map.len();
  let col_count = platform_map[0].len();

  for row_index in 0..row_count {
    for col_index in 0..col_count {
      if platform_map[row_index][col_index] != '.' {
        continue;
      }

      for i in (row_index + 1)..row_count {
        match platform_map[i][col_index] {
          'O' => {
            platform_map[row_index][col_index] = 'O';
            platform_map[i][col_index] = '.';
            break;
          },
          '#' => {
            break;
          },
          _ => {},
        }
      }
    }
  }

  platform_map
}

fn tilt_west(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
  let row_count = platform_map.len();
  let col_count = platform_map[0].len();

  for col_index in 0..col_count {
    for row_index in 0..row_count {
      if platform_map[row_index][col_index] != '.' {
        continue;
      }

      for i in (col_index + 1)..col_count {
        match platform_map[row_index][i] {
          'O' => {
            platform_map[row_index][col_index] = 'O';
            platform_map[row_index][i] = '.';
            break;
          },
          '#' => {
            break;
          },
          _ => {},
        }
      }
    }
  }

  platform_map
}

fn tilt_south(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
  let row_count = platform_map.len();
  let col_count = platform_map[0].len();

  for row_index in (0..row_count).rev() {
    for col_index in 0..col_count {
      if platform_map[row_index][col_index] != '.' {
        continue;
      }

      for i in (0..row_index).rev() {
        match platform_map[i][col_index] {
          'O' => {
            platform_map[row_index][col_index] = 'O';
            platform_map[i][col_index] = '.';
            break;
          },
          '#' => {
            break;
          },
          _ => {},
        }
      }
    }
  }

  platform_map
}

fn tilt_east(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
  let row_count = platform_map.len();
  let col_count = platform_map[0].len();

  for col_index in (0..col_count).rev() {
    for row_index in 0..row_count {
      if platform_map[row_index][col_index] != '.' {
        continue;
      }

      for i in (0..col_index).rev() {
        match platform_map[row_index][i] {
          'O' => {
            platform_map[row_index][col_index] = 'O';
            platform_map[row_index][i] = '.';
            break;
          },
          '#' => {
            break;
          },
          _ => {},
        }
      }
    }
  }

  platform_map
}

fn cycle_tilt(mut platform_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
  platform_map = tilt_north(platform_map);
  platform_map = tilt_west(platform_map);
  platform_map = tilt_south(platform_map);
  platform_map = tilt_east(platform_map);

  platform_map
}

fn stringify_platform_map(platform_map: &Vec<Vec<char>>) -> String {
  platform_map.iter().map(|row| row.iter().collect::<String>() + "\n").collect::<String>()
}

fn find_cycle_loop(mut platform_map: Vec<Vec<char>>) -> (Vec<Vec<char>>, usize, usize) {
  let mut cache = HashMap::<String, usize>::new();

  for index in 0..1000000000 {
    platform_map = cycle_tilt(platform_map);

    let stringified = stringify_platform_map(&platform_map);
    if let Some(&previous_index) = cache.get(&stringified) {
      return (platform_map, previous_index, index);
    } else {
      cache.insert(stringified, index);
    }
  }

  (platform_map, 0, 0)
}

fn part1(contents: &String) -> Result<usize, String> {
  let platform_map = parse_platform_map(contents);

  let platform_map = tilt_north(platform_map);

  Ok(calculate_total_load(&platform_map))
}

fn part2(contents: &String) -> Result<usize, String> {
  let platform_map = parse_platform_map(contents);
  let (mut platform_map, previous_index, loop_index) = find_cycle_loop(platform_map);

  if previous_index != loop_index {
    let diff = loop_index - previous_index;
    let cycles_done = 1000000000 - previous_index - 1;
    let cycles_left = cycles_done - ((cycles_done - previous_index - 1) / diff) * diff;

    for _ in 0..cycles_left {
      platform_map = cycle_tilt(platform_map);
    }
  }

  Ok(calculate_total_load(&platform_map))
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