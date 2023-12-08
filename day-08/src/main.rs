use std::{fs, collections::HashMap};

enum Direction {
  Left,
  Right,
}

fn extract_navigation_instructions(contents: &String) -> Result<Vec<Direction>, String> {
  match contents.lines().next() {
    Some(line) => {
      line
        .chars()
        .map(|char| {
          match char {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(format!("invalid navigation instruction \"{}\"", char)),
          }
        })
        .into_iter()
        .collect()
    },
    None => {
      Err(String::from("unable to extract navigation instructions, end of file"))
    },
  }
}

fn partition_node_line<'a>(line: &'a str) -> (&'a str, &'a str, &'a str) {
  (&line[0..3], &line[7..10], &line[12..15])
}

#[derive(Debug)]
struct Node<'a> {
  key: &'a str,
  left_index: usize,
  right_index: usize,
  is_start: bool,
  is_end: bool,
}

fn make_navigation_tree<'a>(contents: &'a String) -> Result<Vec<Node<'a>>, String> {
  let mut index_map = HashMap::new();
  let mut transient_list = Vec::new();

  let mut lines = contents
    .lines()
    .skip(1)
    .filter(|&line| !line.is_empty())
    .enumerate();

  while let Some((index, line)) = lines.next() {
    let (key, left, right) = partition_node_line(line);

    transient_list.push((key, left, right, key.ends_with('A'), key.ends_with('Z')));
    index_map.insert(key, index);
  }

  let navigation_tree = transient_list
    .iter()
    .map(|&(key, left, right, is_start, is_end)| {
      if let Some(&left_index) = index_map.get(left) {
        if let Some(&right_index) = index_map.get(right) {
          Ok(Node{key, left_index, right_index, is_start, is_end})
        } else {
          Err(format!("unable to make navigation tree, unknown right node \"{}\"", right))
        }
      } else {
        Err(format!("unable to make navigation tree, unknown left node \"{}\"", left))
      }
    })
    .into_iter()
    .collect();

  let navigation_tree = match navigation_tree {
    Ok(navigation_tree) => navigation_tree,
    Err(error) => return Err(error),
  };

  Ok(navigation_tree)
}

fn navigate_tree<'a>(navigation_instructions: &Vec<Direction>, aaa_index: usize, zzz_index: usize, navigation_tree: &Vec<Node<'a>>) -> usize {
  let mut i = 0usize;
  let mut next_node_index = aaa_index;

  loop {
    next_node_index = match &navigation_instructions[i % navigation_instructions.len()] {
      Direction::Left => navigation_tree[next_node_index].left_index,
      Direction::Right => navigation_tree[next_node_index].right_index,
    };

    i = i + 1;

    if next_node_index == zzz_index {
      return i;
    }
  }
}

fn steps_to_next_end_node<'a>(navigation_instructions: &Vec<Direction>, navigation_tree: &Vec<Node<'a>>, start_node_index: usize) -> usize {
  let mut i = 0usize;
  let mut next_node_index = start_node_index;

  loop {
    next_node_index = match &navigation_instructions[i % navigation_instructions.len()] {
      Direction::Left => navigation_tree[next_node_index].left_index,
      Direction::Right => navigation_tree[next_node_index].right_index,
    };

    i = i + 1;

    if navigation_tree[next_node_index].is_end {
      return i;
    }
  }
}

fn gdc(a: usize, b: usize) -> usize {
  let mut a = a;
  let mut b = b;

  while a != b {
    if a > b {
      a = a - b;
    } else {
      b = b - a;
    }
  }

  a
}

fn navigate_tree_as_ghosts<'a>(navigation_instructions: &Vec<Direction>, navigation_tree: &Vec<Node<'a>>) -> Result<usize, String> {
  let steps = navigation_tree
    .iter()
    .enumerate()
    .filter(|(_, node)| node.is_start)
    .map(|(start_node_index, _)| steps_to_next_end_node(navigation_instructions, navigation_tree, start_node_index))
    .collect::<Vec<usize>>();

  if steps.len() == 0 {
    return Err(String::from("unable to navigate tree as ghosts, no staring nodes found"));
  }

  let lcm: usize = steps[1..]
    .iter()
    .fold(steps[0], |acc, &step| acc * (step / gdc(acc, step)));

  Ok(lcm)
}

fn part1(contents: &String) -> Result<usize, String> {
  let navigation_instructions = match extract_navigation_instructions(contents) {
    Ok(navigation_instructions) => navigation_instructions,
    Err(error) => return Err(error),
  };

  let navigation_tree = match make_navigation_tree(contents) {
    Ok(navigation_tree) => navigation_tree,
    Err(error) => return Err(error),
  };

  let aaa_index = match navigation_tree.iter().enumerate().find(|(_, node)| node.key == "AAA") {
    Some((aaa_index, _)) => aaa_index,
    None => return Err(String::from("unable to find index of AAA node")),
  };
  let zzz_index = match navigation_tree.iter().enumerate().find(|(_, node)| node.key == "ZZZ") {
    Some((zzz_index, _)) => zzz_index,
    None => return Err(String::from("unable to find index of ZZZ node")),
  };

  Ok(navigate_tree(&navigation_instructions, aaa_index, zzz_index, &navigation_tree))
}

fn part2(contents: &String) -> Result<usize, String> {
  let navigation_instructions = match extract_navigation_instructions(contents) {
    Ok(navigation_instructions) => navigation_instructions,
    Err(error) => return Err(error),
  };

  let navigation_tree = match make_navigation_tree(contents) {
    Ok(navigation_tree) => navigation_tree,
    Err(error) => return Err(error),
  };

  navigate_tree_as_ghosts(&navigation_instructions, &navigation_tree)
}

fn main() {
  let file_contents = fs::read_to_string("input.txt");

  match file_contents {
    Ok(contents) => {
      match part1(&contents) {
        Ok(result) => println!("part1: {}", result),
        Err(error) => println!("part1: {}", error),
      }

      match part2(&contents) {
        Ok(result) => println!("part2: {}", result),
        Err(error) => println!("part2: {}", error),
      }
    },
    Err(error) => {
      panic!("file not found: {}", error);
    },
  }
}
