use std::fs;

fn hash(plaintext: &str) -> usize {
  plaintext
    .chars()
    .map(|char| char as usize)
    .fold(0, |acc, char| ((acc + char) * 17) % 256)
}

enum Step<'a> {
  Upsert((&'a str, usize, usize)),
  Remove((&'a str, usize)),
}

fn parse_steps<'a >(contents: &'a String) -> Result<Vec<Step<'a>>, String> {
  contents
    .split(',')
    .map(|step| {
      if let Some(sign_pos) = step.find('=') {
        if let Ok(focal_length) = step[(sign_pos + 1)..].parse::<usize>() {
          Ok(Step::Upsert((&step[0..sign_pos], hash(&step[0..sign_pos]), focal_length)))
        } else {
          Err(format!("unable to parse focal length of step \"{step}\""))
        }
      } else if let Some(sign_pos) = step.find('-') {
        Ok(Step::Remove((&step[0..sign_pos], hash(&step[0..sign_pos]))))
      } else {
        Err(format!("unable to parse step \"{step}\""))
      }
    })
    .into_iter()
    .collect()
}

fn part1(contents: &String) -> Result<usize, String> {
  let hash_sum = contents
    .split(',')
    .map(|part| hash(part))
    .sum::<usize>();

  Ok(hash_sum)
}

fn part2(contents: &String) -> Result<usize, String> {
  let steps = match parse_steps(contents) {
    Ok(steps) => steps,
    Err(error) => return Err(error),
  };

  let mut boxes = vec![Vec::<(&str, usize)>::new(); 256];
  for step in steps {
    match step {
      Step::Upsert((label, hash, focal_length)) => {
        if let Some(index)  = boxes[hash].iter().position(|current| current.0 == label) {
          boxes[hash][index].1 = focal_length;
        } else {
          boxes[hash].push((label, focal_length));
        }
      },
      Step::Remove((label, hash)) => {
        let mut index = 0;
        while index < boxes[hash].len() {
          if boxes[hash][index].0 == label {
            boxes[hash].remove(index);
          } else {
            index = index + 1;
          }
        }
      },
    }
  }

  let focusing_power = boxes
    .iter()
    .enumerate()
    .map(|(container_index, container)| {
      container
        .iter()
        .enumerate()
        .map(|(lens_index, (_, focal_length))| (container_index + 1) * (lens_index + 1) * focal_length)
        .sum::<usize>()
    })
    .sum::<usize>();

  Ok(focusing_power)
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