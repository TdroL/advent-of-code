use std::{fs, collections::{HashSet, VecDeque, HashMap}};

fn parse_trials_map(contents: &String) -> Vec<Vec<char>> {
  contents
    .trim()
    .lines()
    .map(|line| {
      line
        .chars()
        .collect()
    })
    .collect()
}

fn find_openning_column(row: &Vec<char>) -> Result<usize, String> {
  match row.iter().position(|&c| c == '.') {
    Some(col) => Ok(col),
    None => Err("unable to find openning".into())
  }
}

static DIRECTIONS: [(isize, isize); 4] = [
  (1, 0),
  (0, -1),
  (-1, 0),
  (0, 1),
];

fn get_forced_dir_index(value: char) -> Option<usize> {
  match value {
    'v' => Some(0),
    '<' => Some(1),
    '^' => Some(2),
    '>' => Some(3),
    _ => None,
  }
}

fn next_coord(coords: &(usize, usize), edges: &(usize, usize), dir_index: usize) -> Option<(usize, (usize, usize))> {
  let dir_index = dir_index % DIRECTIONS.len();
  let direction = &DIRECTIONS[dir_index];

  let row = coords.0 as isize + direction.0;
  let col = coords.1 as isize + direction.1;

  if row < 0 || col < 0 {
    return  None;
  }

  let row = row as usize;
  let col = col as usize;

  if row >= edges.0 || col >= edges.1 {
    return  None;
  }

  Some((dir_index, (row, col)))
}

fn find_next_node(trials_map: &Vec<Vec<char>>, coord: &(usize, usize), dir_index: usize) -> Vec<(usize, (usize, usize))> {
  let row_count = trials_map.len();
  let col_count = trials_map[coord.0].len();

  [dir_index + 3, dir_index, dir_index + 1]
    .iter()
    .filter_map(|&dir_index| next_coord(&coord, &(row_count, col_count), dir_index))
    .collect()
}

fn build_graph(trials_map: &Vec<Vec<char>>, entrance: (usize, usize)) -> HashMap<(usize, usize), HashSet<(usize, (usize, usize))>> {
  let mut result = HashMap::new();

  let mut queue = VecDeque::new();
  for (next_dir_index, next_coord) in find_next_node(&trials_map, &entrance, 0) {
    if trials_map[next_coord.0][next_coord.1] != '#' {
      queue.push_back((entrance.clone(), next_dir_index, next_coord));
    }
  }

  while let Some((prev_coord, mut dir_index, mut coord)) = queue.pop_front() {
    let mut steps = 0;

    loop {
      steps += 1;

      let next_nodes = find_next_node(&trials_map, &coord, dir_index);

      let next_nodes = next_nodes
        .iter()
        .filter(|&(_, coord)| trials_map[coord.0][coord.1] != '#')
        .collect::<Vec<&(usize, (usize, usize))>>();

      if next_nodes.len() == 0 {
        let current_entry = result
          .entry(prev_coord)
          .or_insert_with(|| HashSet::new());

        current_entry.insert((steps, coord.clone()));
        break;
      }

      if next_nodes.len() > 1 {
        {
          let current_entry = result
            .entry(prev_coord)
            .or_insert_with(|| HashSet::new());

          current_entry.insert((steps, coord.clone()));

          for &(next_dir_index, next_coord) in next_nodes.iter() {
            queue.push_back((coord.clone(), *next_dir_index, next_coord.clone()));
          }
        }

        break;
      }

      let &(next_dir_index, next_coord) = next_nodes[0];

      coord = next_coord;
      dir_index = next_dir_index;

      if let Some(forced_dir_index) = get_forced_dir_index(trials_map[coord.0][coord.1]) {
        if forced_dir_index != dir_index {
          break;
        }
      }
    }
  }

  result
}

fn find_longest_path(graph: &HashMap<(usize, usize), HashSet<(usize, (usize, usize))>>, entrance: &(usize, usize)) -> usize {
  let node = match graph.get(&entrance) {
    Some(node) => node,
    None => return 0,
  };

  node
    .iter()
    .map(|(length, coord)| find_longest_path(&graph, coord) + length)
    .fold(0, |acc, length| acc.max(length))
}

fn build_cyclic_graph(trials_map: &Vec<Vec<char>>, entrance: (usize, usize)) -> HashMap<(usize, usize), HashMap<(usize, usize), usize>> {
  let mut result = HashMap::new();

  let mut queue = VecDeque::new();
  for (next_dir_index, next_coord) in find_next_node(&trials_map, &entrance, 0) {
    if trials_map[next_coord.0][next_coord.1] != '#' {
      queue.push_back((entrance.clone(), next_dir_index, next_coord));
    }
  }

  while let Some((prev_coord, mut dir_index, mut coord)) = queue.pop_front() {
    let mut steps = 0;

    loop {
      let next_nodes = find_next_node(&trials_map, &coord, dir_index);

      let next_nodes = next_nodes
        .iter()
        .filter(|&(_, coord)| trials_map[coord.0][coord.1] != '#')
        .collect::<Vec<&(usize, (usize, usize))>>();

      if next_nodes.len() == 0 {
        let current_entry = result
          .entry(prev_coord)
          .or_insert_with(|| HashMap::new());

        current_entry.insert(coord.clone(), steps);
        break;
      }

      steps += 1;

      if next_nodes.len() > 1 {
        let current_entry = result
          .entry(prev_coord)
          .or_insert_with(|| HashMap::new());

        if let None = current_entry.insert(coord.clone(), steps) {
          for &(next_dir_index, next_coord) in next_nodes.iter() {
            queue.push_back((coord.clone(), *next_dir_index, next_coord.clone()));
          }
        }

        break;
      }

      let &(next_dir_index, next_coord) = next_nodes[0];

      coord = next_coord;
      dir_index = next_dir_index;
    }
  }

  result
}

fn find_longest_path_within_cyclic_graph(graph: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>, entrance: &(usize, usize), exit: &(usize, usize), distance: usize, visited: &HashSet<(usize, usize)>) -> usize {
  if entrance == exit {
    return distance + 1;
  }

  let node = match graph.get(&entrance) {
    Some(node) => node,
    None => return 0,
  };

  let mut visited = visited.clone();
  visited.insert(entrance.clone());

  let mut max_steps = 0;

  for (coord, &steps) in node.iter() {
    if visited.contains(coord) {
      continue;
    }

    let value = find_longest_path_within_cyclic_graph(&graph, coord, &exit, distance + steps, &visited);

    max_steps = max_steps.max(value);
  }

  max_steps
}

fn part1(contents: &String) -> Result<usize, String> {
  let trials_map = parse_trials_map(contents);

  let entrance_col = match find_openning_column(&trials_map[0]) {
    Ok(entrance_col) => entrance_col,
    Err(error) => return Err(format!("entrance not found: {error}")),
  };

  let graph = build_graph(&trials_map, (0, entrance_col));

  Ok(find_longest_path(&graph, &(0, entrance_col)))
}

fn part2(contents: &String) -> Result<usize, String> {
  let trials_map = parse_trials_map(contents);

  let entrance_col = match find_openning_column(&trials_map[0]) {
    Ok(entrance_col) => entrance_col,
    Err(error) => return Err(error),
  };

  let exit_col = match find_openning_column(&trials_map[trials_map.len() - 1]) {
    Ok(entrance_col) => entrance_col,
    Err(error) => return Err(format!("exit not found: {error}")),
  };

  let graph = build_cyclic_graph(&trials_map, (0, entrance_col));

  Ok(find_longest_path_within_cyclic_graph(&graph, &(0, entrance_col), &(trials_map.len() - 1, exit_col), 0, &HashSet::new()))
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