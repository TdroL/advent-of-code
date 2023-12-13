use std::fs;

fn parse_pattern_block(block: &str) -> Vec<Vec<char>> {
  block
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>()
}

fn transform_pattern_block_matrix_to_masks(block_matrix: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
  let mut vertical_masks = Vec::new();
  let mut horizontal_masks = Vec::new();

  for i in 0..block_matrix.len() {
    let mut mask = 0;

    for j in 0..block_matrix[i].len() {
      let value = if block_matrix[i][j] == '#' { 1 } else { 0 };

      mask = mask | (value << j);
    }

    vertical_masks.push(mask);
  }

  for j in 0..block_matrix[0].len() {
    let mut mask = 0;

    for i in 0..block_matrix.len() {
      let value = if block_matrix[i][j] == '#' { 1 } else { 0 };

      mask = mask | (value << i);
    }

    horizontal_masks.push(mask);
  }

  (vertical_masks, horizontal_masks)
}

fn parse_patterns(contents: &String) -> Vec<(Vec<usize>, Vec<usize>)> {
  contents
    .trim()
    .split("\r\n\r\n")
    .map(|block| {
      let (vertical_masks, horizontal_masks) = transform_pattern_block_matrix_to_masks(&parse_pattern_block(block));

      (vertical_masks, horizontal_masks)
    })
    .collect()
}

fn is_mask_mirrored(masks: &Vec<usize>, offset: usize)-> bool {
  let mut i = offset;
  let mut j = offset + 1;

  if j >= masks.len() {
    return false;
  }

  loop {
    if masks[i] != masks[j] {
      return false;
    }

    if i == 0 || j == masks.len() - 1 {
      return true;
    }

    i = i - 1;
    j = j + 1;
  };
}

fn find_mirror_index(masks: &Vec<usize>) -> Option<usize> {
  if masks.len() == 0 {
    return None;
  }

  for i in 0..(masks.len() - 1) {
    if is_mask_mirrored(masks, i) {
      return Some(i);
    }
  }

  None
}

fn is_power_of_two(value: usize) -> bool {
  if value == 0 {
    return false;
  }

  (value & (value - 1)) == 0
}

fn is_mask_almost_mirrored(masks: &Vec<usize>, offset: usize) -> bool {
  let mut i = offset;
  let mut j = offset + 1;
  let mut has_one_failure = false;

  if j >= masks.len() {
    return false;
  }

  loop {
    if masks[i] != masks[j] {
      if !has_one_failure {
        if is_power_of_two(masks[i] ^ masks[j]) {
          has_one_failure = true;
        } else {
          return false;
        }
      } else {
        return false;
      }
    }

    if i == 0 || j == masks.len() - 1 {
      return has_one_failure;
    }

    i = i - 1;
    j = j + 1;
  };
}

fn find_and_fix_smudge(masks: &Vec<usize>) -> Option<usize> {
  if masks.len() == 0 {
    return None;
  }
  for i in 0..(masks.len() - 1) {
    if is_mask_almost_mirrored(masks, i) {
      return Some(i);
    }
  }

  None
}

fn part1(contents: &String) -> Result<usize, String> {
  parse_patterns(contents)
    .iter()
    .map(|(vertical_masks, horizontal_masks)| {
      if let Some(horizontal_index) = find_mirror_index(&horizontal_masks) {
        return Ok(horizontal_index + 1);
      }

      if let Some(vertical_index) = find_mirror_index(&vertical_masks) {
        return Ok((vertical_index + 1) * 100);
      }

      Err("unable to find and fix smudge".into())
    })
    .into_iter()
    .sum()
}

fn part2(contents: &String) -> Result<usize, String> {
  parse_patterns(contents)
    .iter()
    .map(|(vertical_masks, horizontal_masks)| {
      if let Some(horizontal_index) = find_and_fix_smudge(&horizontal_masks) {
        return Ok(horizontal_index + 1);
      }

      if let Some(vertical_index) = find_and_fix_smudge(&vertical_masks) {
        return Ok((vertical_index + 1) * 100);
      }

      Err("unable to find mirror index".into())
    })
    .into_iter()
    .sum()
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