use std::{fs, thread, sync::Arc};

fn extract_seeds(contents: &String) -> Vec<u64> {
  let new_line_pos = contents.find('\n').unwrap();
  let line = contents[..new_line_pos].trim();

  line["seeds: ".len()..]
    .split_whitespace()
    .map(|part| part.parse::<u64>().unwrap())
    .collect()
}

fn extract_seed_ranges(contents: &String) -> Vec<u64> {
  let new_line_pos = contents.find('\n').unwrap();
  let line = contents[..new_line_pos].trim();

  let mut result = Vec::new();

  let mut numbers = line["seeds: ".len()..]
    .split_whitespace()
    .map(|part| part.parse::<u64>().unwrap());

  loop {
    let start = if let Some(value) = numbers.next() {
      value
    } else {
      break;
    };
    let end = if let Some(value) = numbers.next() {
      value
    } else {
      break;
    };

    for i in start..(start + end) {
      result.push(i);
    }
  }

  result
}

fn split_into_blocks<'a>(lines: &'a [&'a str]) -> Vec<&'a [&'a str]> {
  let mut blocks = Vec::new();

  let mut start_index = 0;

  for (index, line) in lines.iter().enumerate() {
    if line.is_empty() {
      if !lines[start_index].starts_with("seeds:") {
        blocks.push(&lines[start_index..index]);
      }

      start_index = index + 1;
    }
  }

  if start_index < lines.len() {
    if start_index != 0 {
      blocks.push(&lines[start_index..lines.len()]);
    }
  }

  blocks
}

struct Transform {
  destination: u64,
  source: u64,
  length: u64,
}

fn extract_mappings(contents: &String) -> Vec<Vec<Transform>> {
  let lines = contents.lines().collect::<Vec<&str>>();

  split_into_blocks(&lines[..])
    .iter()
    .map(|block| {
      block[1..]
        .iter()
        .map(|line| {
          let mut parts = line.split_whitespace();

          let destination = parts.next().unwrap().parse::<u64>().unwrap();
          let source = parts.next().unwrap().parse::<u64>().unwrap();
          let length = parts.next().unwrap().parse::<u64>().unwrap();

          Transform{
            destination,
            source,
            length,
          }
        })
        .collect()
    })
    .collect()
}

fn apply_transform(value: u64, transforms: &Vec<Transform>) -> u64 {
  for Transform{destination, source, length} in transforms {
    if &value >= source {
      let diff = value - source;

      if &diff < length {
        return destination + diff;
      }
    }
  }

  value
}

fn apply_mappings(seed: u64, mappings: &Vec<Vec<Transform>>) -> u64 {
  mappings
    .iter()
    .fold(seed, |acc, mapping| apply_transform(acc, mapping))
}

fn fair_division(value: usize, divisor: usize) -> Vec<(usize, usize)> {
  let mut result = vec![0; divisor];

  let mut excess_value = value;

  while excess_value > 0 {
    let partial_value = excess_value / divisor;

    if partial_value != 0 {
      for index in 0..result.len() {
        result[index] = result[index] + partial_value;
      }

      excess_value = excess_value - partial_value * divisor;
    } else {
      for index in 0..excess_value {
        result[index] = result[index] + 1;
      }

      excess_value = 0;
    }
  }

  let mut current_sum = 0usize;

  result
    .iter()
    .map(|&value| {
      let offset = current_sum;
      current_sum = current_sum + value;

      (offset, current_sum)
    })
    .collect()
}

fn part1(contents: &String) -> u64 {
  let seeds = extract_seeds(contents);
  let mappings = extract_mappings(contents);

  seeds
    .iter()
    .map(|seed| apply_mappings(*seed, &mappings))
    .min()
    .unwrap()
}

fn part2(contents: &String) -> u64 {
  let seeds = Arc::new(extract_seed_ranges(contents));
  let mappings = Arc::new(extract_mappings(contents));

  let concurrency = thread::available_parallelism().unwrap().get();
  let work_divisions = fair_division(seeds.len(), concurrency);

  let mut handles = Vec::new();

  for (start, end) in work_divisions {
    let local_seeds = Arc::clone(&seeds);
    let local_mappings = Arc::clone(&mappings);

    handles.push(thread::spawn(move || {
      let mut min = u64::MAX;

      for i in start..end {
        min = u64::min(min, apply_mappings(local_seeds[i], &local_mappings));
      }

      min
    }));
  }

  let mut min = u64::MAX;
  for handle in handles {
    min = u64::min(min, handle.join().unwrap_or(u64::MAX));
  }

  min
}

fn main() {
  let file_contents = fs::read_to_string("input.txt");

  match file_contents {
    Ok(contents) => {
      println!("part1: {}", part1(&contents));
      println!("part2: {}", part2(&contents));
    },
    Err(error) => {
      println!("file not found: {}", error);
    },
  }
}