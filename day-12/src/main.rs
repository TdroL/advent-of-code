use std::{fs, collections::HashMap};

// operational (.)
// damaged (#)
// unknown (?)

fn parse_record(line: &str) -> Result<(&str, Vec<usize>), String> {
  let space_pos = match line.find(' ') {
    Some(space_pos) => space_pos,
    None => return Err(format!("unable to parse line \"{}\"", line)),
  };

  let (conditions, groups) = line.split_at(space_pos);
  let groups = groups
    .trim()
    .split(',')
    .map(|grouping| {
      grouping.parse::<usize>()
        .map_err(|error| format!("unable to parse grouping \"{}\": {}", grouping, error))
    })
    .into_iter()
    .collect();

  match groups {
    Ok(groups) => Ok((conditions, groups)),
    Err(error) => Err(error)
  }
}

fn parse_records(contents: &String) -> Result<Vec<(&str, Vec<usize>)>, String> {
  contents
    .lines()
    .map(|line| parse_record(line))
    .into_iter()
    .collect()
}

fn conditions_has_part(conditions: &str, part: char) -> bool {
  conditions.chars().any(|symbol| symbol == part)
}

fn possible_arrangements_for_conditions(conditions: &str, group: usize) -> usize {
  if conditions.len() < group {
    return 0;
  }

  let mut arragements = 0;

  for offset in 0..=(conditions.len() - group) {
    let (left, conditions) = conditions.split_at(offset);
    let (middle, right) = conditions.split_at(group);

    if !conditions_has_part(left, '#') && !conditions_has_part(middle, '.') && !conditions_has_part(right, '#') {
      arragements = arragements + 1;
    }
  }

  arragements
}

fn min_groups_occupancy(groups: &[usize]) -> usize {
  if groups.len() == 0 {
    return 0;
  }

  groups.iter().sum::<usize>() + groups.len() - 1
}

fn find_possible_arragements(conditions: &str, groups: &[usize], cache: &mut HashMap<(usize, usize, usize), usize>) -> usize {
  if groups.len() == 0 {
    return 0;
  }

  if groups.len() == 1 {
    return possible_arrangements_for_conditions(conditions, groups[0]);
  }

  let group = groups[0];

  let mut offset = 0;
  let min_rest_groups_occupancy = min_groups_occupancy(&groups[1..]);
  let max_offset = conditions.len() - (group + 1 + min_rest_groups_occupancy);

  let mut arragements = 0;
  while offset <= max_offset {
    let position = match conditions.chars().skip(offset).position(|char| char == '#' || char == '?') {
      Some(position) => position,
      None => break,
    };

    offset = offset + position;

    if offset > max_offset {
      break;
    }

    let (left_conditions, rest_conditions) = conditions.split_at(offset);
    if conditions_has_part(left_conditions, '#') {
      break;
    }

    offset = offset + 1;

    let (middle_conditions, rest_conditions) = rest_conditions.split_at(group);
    if conditions_has_part(middle_conditions, '.') {
      continue;
    }

    if rest_conditions.len() == 0 {
      arragements = arragements + 1;
      break;
    }

    let (separator_conditions, rest_conditions) = rest_conditions.split_at(1);
    if conditions_has_part(separator_conditions, '#') {
      continue;
    }

    if groups.len() > 1 && rest_conditions.len() >= min_rest_groups_occupancy {
      let key = (rest_conditions.as_ptr() as *const _ as usize, &groups[1..].as_ptr() as *const _ as usize, groups.len() - 1);

      if let Some(cached_possible_arragements) = cache.get(&key) {
        arragements = arragements + cached_possible_arragements;
      } else {
        let possible_arragements = find_possible_arragements(rest_conditions, &groups[1..], cache);

        cache.insert(key, possible_arragements);

        arragements = arragements + possible_arragements;
      }
    } else {
      break;
    }
  }

  arragements
}

fn unfold_record(conditions: &str, groups: &[usize]) -> (String, Vec<usize>) {
  let mut unfolded_conditions = String::with_capacity(conditions.len() * 5 + 4);

  unfolded_conditions.insert_str(0, conditions);
  unfolded_conditions.insert(conditions.len(), '?');

  unfolded_conditions.insert_str(conditions.len() + 1, conditions);
  unfolded_conditions.insert(conditions.len() * 2 + 1, '?');

  unfolded_conditions.insert_str(conditions.len() * 2 + 2, conditions);
  unfolded_conditions.insert(conditions.len() * 3 + 2, '?');

  unfolded_conditions.insert_str(conditions.len() * 3 + 3, conditions);
  unfolded_conditions.insert(conditions.len() * 4 + 3, '?');

  unfolded_conditions.insert_str(conditions.len() * 4 + 4, conditions);

  let mut unfolded_groups = Vec::with_capacity(groups.len() * 5);
  for _ in 0..5 {
    for &group in groups {
      unfolded_groups.push(group);
    }
  }

  (unfolded_conditions, unfolded_groups)
}

fn part1(contents: &String) -> Result<usize, String> {
  let records = match parse_records(contents) {
    Ok(values) => values,
    Err(error) => return Err(error),
  };

  let sum = records
    .iter()
    .map(|(conditions, groups)| {
      let mut cache = HashMap::new();
      let arragements = find_possible_arragements(conditions, &groups[..], &mut cache);

      arragements
    })
    .sum::<usize>();

  Ok(sum)
}

fn part2(contents: &String) -> Result<usize, String> {
  let records = match parse_records(contents) {
    Ok(values) => values,
    Err(error) => return Err(error),
  };

  let sum = records
    .iter()
    .map(|(conditions, groups)| unfold_record(conditions, groups))
    .map(|(conditions, groups)| {
      let mut cache = HashMap::new();
      let arragements = find_possible_arragements(&conditions[..], &groups[..], &mut cache);

      arragements
    })
    .sum::<usize>();

  Ok(sum)
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