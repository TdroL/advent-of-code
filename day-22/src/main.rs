use std::{fs, collections::{HashSet, VecDeque}};

struct Brick {
  start: [usize; 3],
  end: [usize; 3],
}

fn parse_bricks(contents: &String) -> Result<Vec<Brick>, String> {
  contents
    .trim()
    .lines()
    .map(|line| {
      let (left, right) = match line.split_once('~') {
        Some((left, right)) => (left, right),
        None => return Err(format!("unable to parse line \"{line}\" - separator \"~\" not found")),
      };

      let mut left = left.split(',');
      let mut right = right.split(',');

      let mut start = [0; 3];
      let mut end = [0; 3];

      let mut i = 0;
      while let Some(value) = left.next() {
        if i >= 3 {
          return Err(format!("unable to parse line \"{line}\" - unexpected value \"{value}\""));
        }

        start[i] = match value.parse::<usize>() {
          Ok(value) => value,
          Err(error) => return Err(format!("unable to parse line \"{line}\" - {error}")),
        };

        i += 1;
      }

      if i < 3 {
        return Err(format!("unable to parse line \"{line}\" - expected 3 values, got \"{i}\""));
      }

      let mut i = 0;
      while let Some(value) = right.next() {
        if i >= 3 {
          return Err(format!("unable to parse line \"{line}\" - unexpected value \"{value}\""));
        }

        end[i] = match value.parse::<usize>() {
          Ok(value) => value,
          Err(error) => return Err(format!("unable to parse line \"{line}\" - {error}")),
        };

        i += 1;
      }

      if i < 3 {
        return Err(format!("unable to parse line \"{line}\" - expected 3 values, got \"{i}\""));
      }

      for i in 0..3 {
        start[i] = start[i].min(end[i]);
        end[i] = start[i].max(end[i]);
      }

      Ok(Brick{
        start,
        end,
      })
    })
    .into_iter()
    .collect()
}

fn sort_bricks_by_z(mut bricks: Vec<Brick>) -> Vec<Brick> {
  bricks.sort_by(|a, b| a.end[2].cmp(&b.end[2]));

  bricks
}

fn find_z_buffer_extents(bricks: &Vec<Brick>) -> (usize, usize, usize) {
  let mut extents = (0, 0, 0);

  for brick in bricks.iter() {
    extents.0 = extents.0.max(brick.start[0] + 1);
    extents.1 = extents.1.max(brick.start[1] + 1);
    extents.2 = extents.2.max(brick.start[2] + 1);
  }

  extents
}

fn find_highest_z(z_buffer: &Vec<Vec<(usize, usize)>>, x: (usize, usize), y: (usize, usize)) -> usize {
  let mut acc = 0;

  for row in x.0..=x.1 {
    for col in y.0..=y.1 {
      acc = acc.max(z_buffer[row][col].0);
    }
  }

  acc
}

fn apply_gravity(mut bricks: Vec<Brick>) -> (Vec<Brick>, HashSet<(usize, usize)>) {
  let extents = find_z_buffer_extents(&bricks);

  let mut z_buffer = (0..=extents.0)
    .map(|_| vec![(0, usize::MAX); extents.1 + 1])
    .collect::<Vec<Vec<(usize, usize)>>>();

  let mut collissions = HashSet::new();

  for index in 0..bricks.len() {
    let x = (bricks[index].start[0], bricks[index].end[0]);
    let y = (bricks[index].start[1], bricks[index].end[1]);

    let z = find_highest_z(&z_buffer, x, y);

    let diff: usize = bricks[index].end[2] - bricks[index].start[2];

    bricks[index].start[2] = z + 1;
    bricks[index].end[2] = bricks[index].start[2] + diff;

    for row in x.0..=x.1 {
      for col in y.0..=y.1 {
        if z_buffer[row][col].0 == z && z_buffer[row][col].1 != usize::MAX {
          collissions.insert((z_buffer[row][col].1, index));
        }

        z_buffer[row][col].0 = bricks[index].end[2];
        z_buffer[row][col].1 = index;
      }
    }
  }

  (bricks, collissions)
}

fn collect_brick_collission_counts(bricks: &Vec<Brick>, collissions: &HashSet<(usize, usize)>) -> Vec<usize> {
  let mut results = vec![0; bricks.len()];

  for &(_, index_above) in collissions.iter() {
    results[index_above] += 1;
  }

  results
}

fn group_by(bricks: &Vec<Brick>, collissions: &HashSet<(usize, usize)>, key_getter: fn(&(usize, usize)) -> usize, value_getter: fn(&(usize, usize)) -> usize) -> Vec<Vec<usize>> {
  let mut results = (0..bricks.len())
    .map(|_| vec![])
    .collect::<Vec<Vec<usize>>>();

  for entry in collissions.iter() {
    results[key_getter(entry)].push(value_getter(entry));
  }

  results
}

fn count_nonessensial_bricks(bricks: Vec<Brick>) -> usize {
  let bricks = sort_bricks_by_z(bricks);
  let (bricks, collisions) = apply_gravity(bricks);

  let brick_collission_counts = collect_brick_collission_counts(&bricks, &collisions);
  let below_to_above_mapping = group_by(&bricks, &collisions, |&(index_below, _)| index_below, |&(_, index_above)| index_above);

  let mut count = 0;

  for indices_above in below_to_above_mapping.iter() {
    if indices_above.iter().all(|&index_above| brick_collission_counts[index_above] > 1) {
      count += 1;
    }
  }

  count
}

fn count_fallen_bricks(index: usize, below_to_above_mapping: &Vec<Vec<usize>>, above_to_below_mapping: &Vec<Vec<usize>>, brick_collission_counts: &Vec<usize>) -> usize {
  let mut bricks_removed = vec![false; brick_collission_counts.len()];
  bricks_removed[index] = true;

  let mut queue = VecDeque::new();
  queue.push_back(index);

  while let Some(index) = queue.pop_front() {
    for &index_above in below_to_above_mapping[index].iter() {
      if above_to_below_mapping[index_above].iter().all(|&index_below| bricks_removed[index_below]) {
        bricks_removed[index_above] = true;

        queue.push_back(index_above);
      }
    }
  }

  bricks_removed
    .iter()
    .filter(|&value| *value)
    .count() - 1
}

fn count_unstable_bricks(bricks: Vec<Brick>) -> usize {
  let bricks = sort_bricks_by_z(bricks);
  let (bricks, collisions) = apply_gravity(bricks);

  let brick_collission_counts = collect_brick_collission_counts(&bricks, &collisions);
  let below_to_above_mapping = group_by(&bricks, &collisions, |&(index_below, _)| index_below, |&(_, index_above)| index_above);
  let above_to_below_mapping = group_by(&bricks, &collisions, |&(_, index_above)| index_above, |&(index_below, _)| index_below);

  let mut count = 0;
  for (index, indices_above) in below_to_above_mapping.iter().enumerate() {
    if indices_above.iter().any(|&index_above| brick_collission_counts[index_above] <= 1) {
      count += count_fallen_bricks(index, &below_to_above_mapping, &above_to_below_mapping, &brick_collission_counts);
    }
  }


  count
}

fn part1(contents: &String) -> Result<usize, String> {
  let bricks = match parse_bricks(contents) {
    Ok(bricks) => bricks,
    Err(error) => return Err(error),
  };

  Ok(count_nonessensial_bricks(bricks))
}

fn part2(contents: &String) -> Result<usize, String> {
  let bricks = match parse_bricks(contents) {
    Ok(bricks) => bricks,
    Err(error) => return Err(error),
  };

  Ok(count_unstable_bricks(bricks))
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