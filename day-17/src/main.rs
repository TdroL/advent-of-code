use std::{fs, cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap}};

fn parse_heat_loss_map(contents: &String) -> Result<Vec<Vec<u32>>, String> {
  contents
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|char| {
          match char.to_digit(10) {
            Some(value) => Ok(value),
            None => Err(format!("unable to parse character \"{char}\" as a number"))
          }
        })
        .into_iter()
        .collect()
    })
    .into_iter()
    .collect()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Eq, PartialEq, Hash)]
struct PartialNode {
  row: usize,
  col: usize,
  direction: Direction,
  steps_taken_in_direction: u32,
}

#[derive(Eq)]
struct Node {
  row: usize,
  col: usize,
  direction: Direction,
  steps_taken_in_direction: u32,
  heat_loss: u32,
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
      self.heat_loss == other.heat_loss
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    self.heat_loss.cmp(&other.heat_loss)
  }
}

fn visit_node(visited_nodes: &mut HashMap<PartialNode, u32>, node: Node) -> Option<Node> {
  let key = PartialNode{
    row: node.row,
    col: node.col,
    direction: node.direction,
    steps_taken_in_direction: node.steps_taken_in_direction,
  };

  if let Some(&visited_distance) = visited_nodes.get(&key) {
    if node.heat_loss < visited_distance {
      visited_nodes.insert(key, node.heat_loss);

      Some(node)
    } else {
      None
    }
  } else {
    visited_nodes.insert(key, node.heat_loss);

    Some(node)
  }
}

fn find_node_neighbors_with_direction_limits(heat_loss_map: &Vec<Vec<u32>>, node: Node) -> Vec<Node> {
  let mut neighbors = Vec::with_capacity(3);

  if node.col > 0 {
    if node.direction == Direction::Left && node.steps_taken_in_direction < 3 {
      neighbors.push(Node{
        row: node.row,
        col: node.col - 1,
        direction: Direction::Left,
        steps_taken_in_direction: node.steps_taken_in_direction + 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row][node.col - 1],
      });
    } else if node.direction == Direction::Up || node.direction == Direction::Down {
      neighbors.push(Node{
        row: node.row,
        col: node.col - 1,
        direction: Direction::Left,
        steps_taken_in_direction: 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row][node.col - 1],
      });
    }
  }

  if node.col + 1 < heat_loss_map[node.row].len() {
    if node.direction == Direction::Right && node.steps_taken_in_direction < 3 {
      neighbors.push(Node{
        row: node.row,
        col: node.col + 1,
        direction: Direction::Right,
        steps_taken_in_direction: node.steps_taken_in_direction + 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row][node.col + 1],
      });
    } else if node.direction == Direction::Up || node.direction == Direction::Down {
      neighbors.push(Node{
        row: node.row,
        col: node.col + 1,
        direction: Direction::Right,
        steps_taken_in_direction: 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row][node.col + 1],
      });
    }
  }

  if node.row > 0 {
    if node.direction == Direction::Up && node.steps_taken_in_direction < 3 {
      neighbors.push(Node{
        row: node.row - 1,
        col: node.col,
        direction: Direction::Up,
        steps_taken_in_direction: node.steps_taken_in_direction + 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row - 1][node.col],
      });
    } else if node.direction == Direction::Left || node.direction == Direction::Right {
      neighbors.push(Node{
        row: node.row - 1,
        col: node.col,
        direction: Direction::Up,
        steps_taken_in_direction: 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row - 1][node.col],
      });
    }
  }

  if node.row + 1 < heat_loss_map.len() {
    if node.direction == Direction::Down && node.steps_taken_in_direction < 3 {
      neighbors.push(Node{
        row: node.row + 1,
        col: node.col,
        direction: Direction::Down,
        steps_taken_in_direction: node.steps_taken_in_direction + 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row + 1][node.col],
      });
    } else if node.direction == Direction::Left || node.direction == Direction::Right {
      neighbors.push(Node{
        row: node.row + 1,
        col: node.col,
        direction: Direction::Down,
        steps_taken_in_direction: 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row + 1][node.col],
      });
    }
  }

  neighbors
}

fn find_shortest_distance_with_direction_limits(heat_loss_map: &Vec<Vec<u32>>) -> u32 {
  if heat_loss_map.len() == 0 || heat_loss_map[0].len() == 0 {
    return 0;
  }

  let goal_row = heat_loss_map.len() - 1;
  let goal_col = heat_loss_map[0].len() - 1;

  let mut visited_nodes = HashMap::new();
  let mut next_nodes = BinaryHeap::new();
  next_nodes.push(Reverse(Node{
    row: 0,
    col: 0,
    direction: Direction::Right,
    steps_taken_in_direction: 1,
    heat_loss: 0,
  }));

  next_nodes.push(Reverse(Node{
    row: 0,
    col: 0,
    direction: Direction::Down,
    steps_taken_in_direction: 1,
    heat_loss: 0,
  }));

  while let Some(Reverse(node)) = next_nodes.pop() {
    if node.row == goal_row && node.col == goal_col {
      // print_path(&heat_loss_map, &node);
      return node.heat_loss;
    }

    if let Some(visited_node) = visit_node(&mut visited_nodes, node) {
      for neighbor_node in find_node_neighbors_with_direction_limits(&heat_loss_map, visited_node) {
        next_nodes.push(Reverse(neighbor_node));
      }
    }
  }

  u32::MAX
}

fn find_node_neighbors_with_turning_limits(heat_loss_map: &Vec<Vec<u32>>, node: Node) -> Vec<Node> {
  let mut neighbors = Vec::with_capacity(3);

  if node.col > 0 {
    if node.direction == Direction::Left && node.steps_taken_in_direction < 10 {
      neighbors.push(Node{
        row: node.row,
        col: node.col - 1,
        direction: Direction::Left,
        steps_taken_in_direction: node.steps_taken_in_direction + 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row][node.col - 1],
      });
    } else if (node.direction == Direction::Up || node.direction == Direction::Down) && node.steps_taken_in_direction >= 4 {
      neighbors.push(Node{
        row: node.row,
        col: node.col - 1,
        direction: Direction::Left,
        steps_taken_in_direction: 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row][node.col - 1],
      });
    }
  }

  if node.col + 1 < heat_loss_map[node.row].len() {
    if node.direction == Direction::Right && node.steps_taken_in_direction < 10 {
      neighbors.push(Node{
        row: node.row,
        col: node.col + 1,
        direction: Direction::Right,
        steps_taken_in_direction: node.steps_taken_in_direction + 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row][node.col + 1],
      });
    } else if (node.direction == Direction::Up || node.direction == Direction::Down) && node.steps_taken_in_direction >= 4 {
      neighbors.push(Node{
        row: node.row,
        col: node.col + 1,
        direction: Direction::Right,
        steps_taken_in_direction: 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row][node.col + 1],
      });
    }
  }

  if node.row > 0 {
    if node.direction == Direction::Up && node.steps_taken_in_direction < 10 {
      neighbors.push(Node{
        row: node.row - 1,
        col: node.col,
        direction: Direction::Up,
        steps_taken_in_direction: node.steps_taken_in_direction + 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row - 1][node.col],
      });
    } else if (node.direction == Direction::Left || node.direction == Direction::Right) && node.steps_taken_in_direction >= 4 {
      neighbors.push(Node{
        row: node.row - 1,
        col: node.col,
        direction: Direction::Up,
        steps_taken_in_direction: 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row - 1][node.col],
      });
    }
  }

  if node.row + 1 < heat_loss_map.len() {
    if node.direction == Direction::Down && node.steps_taken_in_direction < 10 {
      neighbors.push(Node{
        row: node.row + 1,
        col: node.col,
        direction: Direction::Down,
        steps_taken_in_direction: node.steps_taken_in_direction + 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row + 1][node.col],
      });
    } else if (node.direction == Direction::Left || node.direction == Direction::Right) && node.steps_taken_in_direction >= 4 {
      neighbors.push(Node{
        row: node.row + 1,
        col: node.col,
        direction: Direction::Down,
        steps_taken_in_direction: 1,
        heat_loss: node.heat_loss + heat_loss_map[node.row + 1][node.col],
      });
    }
  }

  neighbors
}

fn find_shortest_distance_with_turning_limits(heat_loss_map: &Vec<Vec<u32>>) -> u32 {
  if heat_loss_map.len() == 0 || heat_loss_map[0].len() == 0 {
    return 0;
  }

  let goal_row = heat_loss_map.len() - 1;
  let goal_col = heat_loss_map[0].len() - 1;

  let mut visited_nodes = HashMap::new();
  let mut next_nodes = BinaryHeap::new();
  next_nodes.push(Reverse(Node{
    row: 0,
    col: 0,
    direction: Direction::Right,
    steps_taken_in_direction: 1,
    heat_loss: 0,
  }));

  next_nodes.push(Reverse(Node{
    row: 0,
    col: 0,
    direction: Direction::Down,
    steps_taken_in_direction: 1,
    heat_loss: 0,
  }));

  while let Some(Reverse(node)) = next_nodes.pop() {
    if node.row == goal_row && node.col == goal_col && node.steps_taken_in_direction >= 4 {
      return node.heat_loss;
    }

    if let Some(visited_node) = visit_node(&mut visited_nodes, node) {
      for neighbor_node in find_node_neighbors_with_turning_limits(&heat_loss_map, visited_node) {
        next_nodes.push(Reverse(neighbor_node));
      }
    }
  }

  u32::MAX
}

fn part1(contents: &String) -> Result<u32, String> {
  let heat_loss_map = match parse_heat_loss_map(contents) {
    Ok(heat_loss_map) => heat_loss_map,
    Err(error) => return Err(error),
  };

  Ok(find_shortest_distance_with_direction_limits(&heat_loss_map))
}

fn part2(contents: &String) -> Result<u32, String> {
  let heat_loss_map = match parse_heat_loss_map(contents) {
    Ok(heat_loss_map) => heat_loss_map,
    Err(error) => return Err(error),
  };

  Ok(find_shortest_distance_with_turning_limits(&heat_loss_map))
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