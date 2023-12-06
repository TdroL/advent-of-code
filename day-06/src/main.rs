use std::fs;

fn extract_times(line: &str) -> Vec<u64> {
  let colon_pos = line.find(':').unwrap();

  line[(colon_pos + ":".len())..]
    .split_whitespace()
    .map(|part| part.parse::<u64>().unwrap())
    .collect()
}

fn extract_distances(line: &str) -> Vec<u64> {
  let colon_pos = line.find(':').unwrap();

  line[(colon_pos + ":".len())..]
    .split_whitespace()
    .map(|part| part.parse::<u64>().unwrap())
    .collect()
}

fn extract_times_ignoring_kerning(line: &str) -> u64 {
  let colon_pos = line.find(':').unwrap();

  line[(colon_pos + ":".len())..]
    .split_whitespace()
    .fold(String::new(), |acc, part| acc + part)
    .parse::<u64>()
    .unwrap()
}

fn extract_distance_ignoring_kerning(line: &str) -> u64 {
  let colon_pos = line.find(':').unwrap();

  line[(colon_pos + ":".len())..]
    .split_whitespace()
    .fold(String::new(), |acc, part| acc + part)
    .parse::<u64>()
    .unwrap()
}

fn calculate_margin_of_error(time: u64, record_distance: u64) -> u64 {
  let mut low_end = 0;
  let mut high_end = 0;

  for i in 1..(time - 1) {
    let speed = i;
    let time_left = time - i;

    let travel_distance = speed * time_left;

    if travel_distance > record_distance {
      low_end = i;
      break;
    }
  }

  for i in (1..(time - 1)).rev() {
    let speed = i;
    let time_left = time - i;

    let travel_distance = speed * time_left;

    if travel_distance > record_distance {
      high_end = i;
      break;
    }
  }

  high_end - low_end + 1
}

fn part1(contents: &String) -> u64 {
  let mut lines = contents.lines();

  let times = extract_times(lines.next().unwrap());
  let distances = extract_distances(lines.next().unwrap());

  (0..times.len())
    .map(|index| {
      let time = times[index];
      let record_distance = distances[index];

      calculate_margin_of_error(time, record_distance)
    })
    .reduce(|acc, value| acc * value)
    .unwrap_or(0)
}

fn part2(contents: &String) -> u64 {
  let mut lines = contents.lines();

  let time = extract_times_ignoring_kerning(lines.next().unwrap());
  let record_distance = extract_distance_ignoring_kerning(lines.next().unwrap());

  calculate_margin_of_error(time, record_distance)
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