use std::fs;

fn parse_galaxies(contents: &String) -> (Vec<(usize, usize)>, usize, usize) {
  let mut galaxies = vec![];

  let rows = contents.lines().count();
  let cols = contents.lines().next().map(|line| line.len()).unwrap_or(0);

  for (row, line) in contents.lines().enumerate() {
    for (col, symbol) in line.chars().enumerate() {
      if symbol == '#' {
        galaxies.push((row, col));
      }
    }
  }

  (galaxies, rows, cols)
}

fn expand_space(galaxies: Vec<(usize, usize)>, expansion_rate: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
  let mut spaces_vertical = vec![true; rows];
  let mut spaces_horizontal = vec![true; cols];

  for &(row, col) in galaxies.iter() {
    spaces_vertical[row] = false;
    spaces_horizontal[col] = false;
  }

  let mut summed_area_spaces_vertical = vec![0; rows];
  let mut summed_area_spaces_horizontal = vec![0; cols];

  let mut current_spaces_vertical_sum = 0;
  for i in 0..summed_area_spaces_vertical.len() {
    if spaces_vertical[i] {
      current_spaces_vertical_sum = current_spaces_vertical_sum + (expansion_rate - 1);
    }

    summed_area_spaces_vertical[i] = current_spaces_vertical_sum;
  }

  let mut current_spaces_horizontal_sum = 0;
  for i in 0..summed_area_spaces_horizontal.len() {
    if spaces_horizontal[i] {
      current_spaces_horizontal_sum = current_spaces_horizontal_sum + (expansion_rate - 1);
    }

    summed_area_spaces_horizontal[i] = current_spaces_horizontal_sum;
  }

  let mut galaxies = galaxies;
  for i in 0..galaxies.len() {
    galaxies[i] = (galaxies[i].0 + summed_area_spaces_vertical[galaxies[i].0], galaxies[i].1 + summed_area_spaces_horizontal[galaxies[i].1]);
  }

  galaxies
}

fn manhatan_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
  (a.0.max(b.0) - a.0.min(b.0)) + (a.1.max(b.1) - a.1.min(b.1))
}

fn calculate_sum_of_shortest_paths(galaxies: Vec<(usize, usize)>) -> usize {
  let mut distance = 0;

  for i in 0..galaxies.len() {
    for j in (i + 1)..galaxies.len() {
      distance = distance + manhatan_distance(&galaxies[i], &galaxies[j]);
    }
  }

  distance
}

fn part1(contents: &String) -> Result<usize, String> {
  let (galaxies, rows, cols) = parse_galaxies(contents);
  let galaxies = expand_space(galaxies, 2, rows, cols);

  Ok(calculate_sum_of_shortest_paths(galaxies))
}

fn part2(contents: &String) -> Result<usize, String> {
  let (galaxies, rows, cols) = parse_galaxies(contents);
  let galaxies = expand_space(galaxies, 1000000, rows, cols);

  Ok(calculate_sum_of_shortest_paths(galaxies))
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