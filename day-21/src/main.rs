use std::{fs, collections::VecDeque};

fn parse_maze(contents: &String) -> Vec<Vec<char>> {
  contents
    .lines()
    .map(|line| {
      line
        .chars()
        .collect()
    })
    .collect()
}

fn find_start_position(maze: &Vec<Vec<char>>) -> Option<(usize, usize)> {
  for row in 0..maze.len() {
    for col in 0..maze[row].len() {
      if maze[row][col] == 'S' {
        return Some((row, col))
      }
    }
  }

  None
}

fn flood_fill(maze: &Vec<Vec<char>>, start_position: (usize, usize), start_step: usize, max_steps: usize) -> Vec<Vec<usize>> {
  let mut steps_map = (0..maze.len())
    .map(|row| vec![usize::MAX; maze[row].len()])
    .collect::<Vec<Vec<usize>>>();

  let mut queue = VecDeque::new();
  queue.push_back((start_position.0, start_position.1, start_step));

  while let Some((row, col, steps)) = queue.pop_front() {
    if steps_map[row][col] <= steps {
      continue;
    }

    steps_map[row][col] = steps;

    if steps == max_steps {
      continue;
    }

    if row > 0 && maze[row - 1][col] == '.' {
      queue.push_back((row - 1, col, steps + 1));
    }

    if row + 1 < maze.len() && maze[row + 1][col] == '.' {
      queue.push_back((row + 1, col, steps + 1));
    }

    if col > 0 && maze[row][col - 1] == '.' {
      queue.push_back((row, col - 1, steps + 1));
    }

    if col + 1 < maze[row].len() && maze[row][col + 1] == '.' {
      queue.push_back((row, col + 1, steps + 1));
    }
  }

  steps_map
}

fn count_even_steps(steps_map: &Vec<Vec<usize>>) -> usize {
  steps_map
    .iter()
    .map(|steps_row| {
      steps_row
        .iter()
        .map(|&value| if value != usize::MAX && value % 2 == 0 { 1 } else { 0 })
        .sum::<usize>()
    })
    .sum()
}

fn count_odd_steps(steps_map: &Vec<Vec<usize>>) -> usize {
  steps_map
    .iter()
    .map(|steps_row| {
      steps_row
        .iter()
        .map(|&value| if value != usize::MAX && value % 2 == 1 { 1 } else { 0 })
        .sum::<usize>()
    })
    .sum()
}

fn part1(contents: &String) -> Result<usize, String> {
  let mut maze = parse_maze(contents);

  let start_position = match find_start_position(&maze) {
    Some(start_position) => start_position,
    None => return Err("unable to find starting position".into()),
  };

  maze[start_position.0][start_position.1] = '.';

  let steps_map = flood_fill(&maze, start_position, 0, 64);

  Ok(count_even_steps(&steps_map))
}

fn part2(contents: &String) -> Result<usize, String> {
  let mut maze = parse_maze(contents);

  let start_position = match find_start_position(&maze) {
    Some(start_position) => start_position,
    None => return Err("unable to find starting position".into()),
  };

  maze[start_position.0][start_position.1] = '.';

  let last_row = maze.len() - 1;
  let last_col = maze[0].len() - 1;

  let steps_map = flood_fill(&maze, start_position, 0, usize::MAX);

  let steps_map_count = count_odd_steps(&steps_map);

  let start_position_s = (0, start_position.1);
  let start_position_n = (last_row, start_position.1);
  let start_position_e = (start_position.0, 0);
  let start_position_w = (start_position.0, last_col);

  let start_position_se = (0, 0);
  let start_position_ne = (last_row, 0);
  let start_position_sw = (0, last_col);
  let start_position_nw = (last_row, last_col);

  let dim = maze.len();

  let steps_map_s_odd = flood_fill(&maze, start_position_s, 1, usize::MAX);
  let steps_map_s_even = flood_fill(&maze, start_position_s, 0, usize::MAX);
  let steps_map_s_odd_count = count_odd_steps(&steps_map_s_odd);
  let steps_map_s_even_count = count_odd_steps(&steps_map_s_even);

  let steps_map_s_edge = flood_fill(&maze, start_position_s, 1, dim);
  let steps_map_s_edge_count = count_odd_steps(&steps_map_s_edge);

  let steps_map_n_odd = flood_fill(&maze, start_position_n, 1, usize::MAX);
  let steps_map_n_even = flood_fill(&maze, start_position_n, 0, usize::MAX);
  let steps_map_n_odd_count = count_odd_steps(&steps_map_n_odd);
  let steps_map_n_even_count = count_odd_steps(&steps_map_n_even);

  let steps_map_n_edge = flood_fill(&maze, start_position_n, 1, dim);
  let steps_map_n_edge_count = count_odd_steps(&steps_map_n_edge);

  let steps_map_e_odd = flood_fill(&maze, start_position_e, 1, usize::MAX);
  let steps_map_e_even = flood_fill(&maze, start_position_e, 0, usize::MAX);
  let steps_map_e_odd_count = count_odd_steps(&steps_map_e_odd);
  let steps_map_e_even_count = count_odd_steps(&steps_map_e_even);

  let steps_map_e_edge = flood_fill(&maze, start_position_e, 1, dim);
  let steps_map_e_edge_count = count_odd_steps(&steps_map_e_edge);

  let steps_map_w_odd = flood_fill(&maze, start_position_w, 1, usize::MAX);
  let steps_map_w_even = flood_fill(&maze, start_position_w, 0, usize::MAX);
  let steps_map_w_odd_count = count_odd_steps(&steps_map_w_odd);
  let steps_map_w_even_count = count_odd_steps(&steps_map_w_even);

  let steps_map_w_edge = flood_fill(&maze, start_position_w, 1, dim);
  let steps_map_w_edge_count = count_odd_steps(&steps_map_w_edge);

  let steps_map_se_odd = flood_fill(&maze, start_position_se, 1, usize::MAX);
  let steps_map_se_even = flood_fill(&maze, start_position_se, 0, usize::MAX);
  let steps_map_se_odd_count = count_odd_steps(&steps_map_se_odd);
  let steps_map_se_even_count = count_odd_steps(&steps_map_se_even);

  let steps_map_se_lesser_edge = flood_fill(&maze, start_position_se, 1, dim / 2);
  let steps_map_se_lesser_edge_count = count_odd_steps(&steps_map_se_lesser_edge);

  let steps_map_se_greater_edge = flood_fill(&maze, start_position_se, 0, dim + (dim / 2) - 1);
  let steps_map_se_greater_edge_count = count_odd_steps(&steps_map_se_greater_edge);

  let steps_map_ne_odd = flood_fill(&maze, start_position_ne, 1, usize::MAX);
  let steps_map_ne_even = flood_fill(&maze, start_position_ne, 0, usize::MAX);
  let steps_map_ne_odd_count = count_odd_steps(&steps_map_ne_odd);
  let steps_map_ne_even_count = count_odd_steps(&steps_map_ne_even);

  let steps_map_ne_lesser_edge = flood_fill(&maze, start_position_ne, 1, dim / 2);
  let steps_map_ne_lesser_edge_count = count_odd_steps(&steps_map_ne_lesser_edge);

  let steps_map_ne_greater_edge = flood_fill(&maze, start_position_ne, 0, dim + (dim / 2) - 1);
  let steps_map_ne_greater_edge_count = count_odd_steps(&steps_map_ne_greater_edge);

  let steps_map_sw_odd = flood_fill(&maze, start_position_sw, 1, usize::MAX);
  let steps_map_sw_even = flood_fill(&maze, start_position_sw, 0, usize::MAX);
  let steps_map_sw_odd_count = count_odd_steps(&steps_map_sw_odd);
  let steps_map_sw_even_count = count_odd_steps(&steps_map_sw_even);

  let steps_map_sw_lesser_edge = flood_fill(&maze, start_position_sw, 1, dim / 2);
  let steps_map_sw_lesser_edge_count = count_odd_steps(&steps_map_sw_lesser_edge);

  let steps_map_sw_greater_edge = flood_fill(&maze, start_position_sw, 0, dim + (dim / 2) - 1);
  let steps_map_sw_greater_edge_count = count_odd_steps(&steps_map_sw_greater_edge);

  let steps_map_nw_odd = flood_fill(&maze, start_position_nw, 1, usize::MAX);
  let steps_map_nw_even = flood_fill(&maze, start_position_nw, 0, usize::MAX);
  let steps_map_nw_odd_count = count_odd_steps(&steps_map_nw_odd);
  let steps_map_nw_even_count = count_odd_steps(&steps_map_nw_even);

  let steps_map_nw_lesser_edge = flood_fill(&maze, start_position_nw, 1, dim / 2);
  let steps_map_nw_lesser_edge_count = count_odd_steps(&steps_map_nw_lesser_edge);

  let steps_map_nw_greater_edge = flood_fill(&maze, start_position_nw, 0, dim + (dim / 2) - 1);
  let steps_map_nw_greater_edge_count = count_odd_steps(&steps_map_nw_greater_edge);

  let steps = 26501365;

  let axis_repeats = (steps - dim / 2) / dim;
  let diagonal_even_repeats = axis_repeats * (axis_repeats - 2) / 4;
  let diagonal_odd_repeats = (axis_repeats - 2) * (axis_repeats - 2) / 4;

  Ok(
    steps_map_count +

    ((axis_repeats - 1) / 2) * (steps_map_s_odd_count + steps_map_n_odd_count + steps_map_e_odd_count + steps_map_w_odd_count)+
    (axis_repeats / 2) * (steps_map_s_even_count + steps_map_n_even_count + steps_map_e_even_count + steps_map_w_even_count) +

    diagonal_even_repeats * (steps_map_se_odd_count + steps_map_ne_odd_count + steps_map_sw_odd_count + steps_map_nw_odd_count) +
    diagonal_odd_repeats * (steps_map_se_even_count + steps_map_ne_even_count + steps_map_sw_even_count + steps_map_nw_even_count) +

    (steps_map_s_edge_count + steps_map_n_edge_count + steps_map_e_edge_count + steps_map_w_edge_count) +
    axis_repeats * (steps_map_se_lesser_edge_count + steps_map_ne_lesser_edge_count + steps_map_sw_lesser_edge_count + steps_map_nw_lesser_edge_count) +
    (axis_repeats - 1) * (steps_map_se_greater_edge_count + steps_map_ne_greater_edge_count + steps_map_sw_greater_edge_count + steps_map_nw_greater_edge_count) +
    0
  )
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