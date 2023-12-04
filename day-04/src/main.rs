use std::fs;

struct LineSegments<'a> {
  winning_numbers: &'a str,
  chosen_numbers: &'a str,
}

fn extract_segments<'a>(line: &'a str) -> LineSegments<'a> {
  let colon_pos = line.find(":").unwrap();
  let pipe_pos = line.find("|").unwrap();

  LineSegments{
    winning_numbers: &line[(colon_pos + ":".len())..pipe_pos],
    chosen_numbers: &line[(pipe_pos + "|".len())..],
  }
}

fn extract_numbers(numbers_segment: &str) -> Vec<u32> {
  numbers_segment.split_whitespace()
    .map(|part| part.parse::<u32>().unwrap())
    .collect::<Vec<u32>>()
}

fn part1(contents: &String) -> u32 {
  contents
    .lines()
    .map(|line| {
      let line_segments = extract_segments(line);
      let winning_numbers = extract_numbers(line_segments.winning_numbers);
      let chosen_numbers = extract_numbers(line_segments.chosen_numbers);

      chosen_numbers
        .iter()
        .fold(0, |acc, chosen_number| {
          let is_winning_number = winning_numbers.iter().any(|winning_number| winning_number == chosen_number);

          if !is_winning_number {
            acc
          } else if acc == 0 {
            1
          } else {
            acc * 2
          }
        })
    })
    .sum::<u32>()
}

fn part2(contents: &String) -> u32 {
  let winning_counts = contents
    .lines()
    .map(|line| {
      let line_segments = extract_segments(line);
      let winning_numbers = extract_numbers(line_segments.winning_numbers);
      let chosen_numbers = extract_numbers(line_segments.chosen_numbers);

      chosen_numbers
        .iter()
        .fold(0usize, |acc, chosen_number| {
          if winning_numbers.iter().any(|winning_number| winning_number == chosen_number) {
            acc + 1
          } else {
            acc
          }
        })
    })
    .collect::<Vec<usize>>();

  let mut scratchcard_counts = Vec::<u32>::new();
  scratchcard_counts.resize(winning_counts.len(), 1);

  let mut i = 0usize;
  while i < scratchcard_counts.len() {
    let mut j = 0usize;
    while j < winning_counts[i] {
      scratchcard_counts[i + j + 1] = scratchcard_counts[i + j + 1] + scratchcard_counts[i];

      j = j + 1;
    }

    i = i + 1;
  }

  scratchcard_counts.iter().sum::<u32>()
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
