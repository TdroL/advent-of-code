use std::fs;

fn part1(contents: &String) -> u32 {
  contents.lines()
    .map(|line| {
      let mut first = 0;
      let mut last = 0;

      let mut chars = line.chars();

      while let Some(char) = chars.next() {
        if char.is_ascii_digit() {
          first = char.to_digit(10).unwrap();
          last = first;
          break;
        }
      }

      while let Some(char) = chars.next() {
        if char.is_ascii_digit() {
          last = char.to_digit(10).unwrap();
        }
      }

      first * 10 + last
    })
    .sum::<u32>()
}

fn part2(contents: &String) -> u32 {
  let spelled_numbers = vec![
    (0, "0"), (1, "1"), (2, "2"), (3, "3"), (4, "4"), (5, "5"), (6, "6"), (7, "7"), (8, "8"), (9, "9"),
    (1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five"), (6, "six"), (7, "seven"), (8, "eight"), (9, "nine"),
  ];

  contents.lines()
    .map(|line| {
      let first_number = spelled_numbers
        .iter()
        .filter_map(|&(value, pattern)| line.find(pattern).map(|pos| (value, pos)))
        .reduce(|acc, tuple| {
          if acc.1 > tuple.1 {
            tuple
          } else {
            acc
          }
        });

      let last_number = spelled_numbers
        .iter()
        .filter_map(|&(value, pattern)| line.rfind(pattern).map(|pos| (value, pos)))
        .reduce(|acc, tuple| {
          if acc.1 < tuple.1 {
            tuple
          } else {
            acc
          }
        });

      match first_number.zip(last_number) {
        Some(((first, _), (last, _))) => first * 10 + last,
        None => 0,
      }
    })
    .sum::<u32>()
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
