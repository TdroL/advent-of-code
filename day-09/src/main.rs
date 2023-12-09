use std::fs;

fn parse_line(line: &str) -> Result<Vec<i64>, String> {
  let values = line.split_whitespace();

  let mut result = Vec::with_capacity(values.clone().count() + 1);

  for part in values {
    let value = match part.parse::<i64>() {
      Ok(value) => value,
      Err(error) => return Err(format!("failed to parse \"{}\" as a number: {}", part, error)),
    };

    result.push(value);
  }

  Ok(result)
}

fn extrapolate_last_value(values: &mut Vec<i64>) -> Result<i64, String> {
  for start_index in 1..values.len() {
    for i in (start_index..values.len()).rev() {
      values[i] = values[i] - values[i - 1];
    }

    let has_only_zeroes = &values[start_index..].iter().all(|&value| value == 0);
    if *has_only_zeroes {
      break;
    }
  }

  values.push(0);

  for start_index in 1..values.len() {
    for i in (start_index..values.len()).rev() {
      values[i] = values[i] + values[i - 1];
    }
  }

  match values.last() {
    Some(&last_value) => Ok(last_value),
    None => Err(String::from("no values")),
  }
}

fn extrapolate_first_value(values: &mut Vec<i64>) -> Result<i64, String> {
  for start_index in 1..values.len() {
    for i in (start_index..values.len()).rev() {
      values[i] = values[i] - values[i - 1];
    }

    let has_only_zeroes = &values[start_index..].iter().all(|&value| value == 0);
    if *has_only_zeroes {
      break;
    }
  }

  let mut last_value = 0;
  for i in (0..values.len()).rev() {
    last_value = values[i] - last_value;
  }

  Ok(last_value)
}

fn part1(contents: &String) -> Result<i64, String> {
  contents
    .lines()
    .map(|line| {
      let mut values = match parse_line(line) {
        Ok(values) => values,
        Err(error) => return Err(error),
      };

      extrapolate_last_value(&mut values)
    })
    .into_iter()
    .sum::<Result<i64, String>>()
}

fn part2(contents: &String) -> Result<i64, String> {
  contents
    .lines()
    .map(|line| {
      let mut values = match parse_line(line) {
        Ok(values) => values,
        Err(error) => return Err(error),
      };

      extrapolate_first_value(&mut values)
    })
    .into_iter()
    .sum::<Result<i64, String>>()
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
