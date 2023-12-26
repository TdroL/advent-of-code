use std::fs;

struct Hailstone  {
  px: f64,
  py: f64,
  pz: f64,
  vx: f64,
  vy: f64,
  vz: f64,
}

fn parse_points(points: &str) -> Result<(f64, f64, f64), String> {
  let mut parts = points.split(',');

  let x = match parts.next() {
    Some(x) => {
      match x.trim().parse::<f64>() {
        Ok(x) => x,
        Err(error) => return Err(format!("unable to parse x value in \"{points}\" - {error}")),
      }
    },
    None => return Err(format!("unable to find x value in \"{points}\"")),
  };

  let y = match parts.next() {
    Some(y) => {
      match y.trim().parse::<f64>() {
        Ok(y) => y,
        Err(error) => return Err(format!("unable to parse y value in \"{points}\" - {error}")),
      }
    },
    None => return Err(format!("unable to find y value in \"{points}\"")),
  };

  let z = match parts.next() {
    Some(z) => {
      match z.trim().parse::<f64>() {
        Ok(z) => z,
        Err(error) => return Err(format!("unable to parse z value in \"{points}\" - {error}")),
      }
    },
    None => return Err(format!("unable to find z value in \"{points}\"")),
  };

  Ok((x, y, z))
}

fn parse_hailstones(contents: &String) -> Result<Vec<Hailstone>, String> {
  contents
    .trim()
    .lines()
    .map(|line| {
      let (left, right) = match line.trim().split_once('@') {
        Some((left, right)) => (left, right),
        None => return Err(format!("unable to parse line \"{line}\" - separator \"@\" not found")),
      };

      let (px, py, pz) = match parse_points(left) {
        Ok((px, py, pz)) => (px, py, pz),
        Err(error) => return Err(format!("unable to parse line \"{line}\" - {error}")),
      };

      let (vx, vy, vz) = match parse_points(right) {
        Ok((vx, vy, vz)) => (vx, vy, vz),
        Err(error) => return Err(format!("unable to parse line \"{line}\" - {error}")),
      };

      Ok(Hailstone{ px, py, pz, vx, vy, vz })
    })
    .into_iter()
    .collect()
}

fn intersection(a: &Hailstone, b: &Hailstone) -> (f64, f64) {
  let x1 = a.px;
  let x2 = a.px + a.vx * 100000000000000f64;

  let y1 = a.py;
  let y2 = a.py + a.vy * 100000000000000f64;

  let x3 = b.px;
  let x4 = b.px + b.vx * 100000000000000f64;

  let y3 = b.py;
  let y4 = b.py + b.vy * 100000000000000f64;

  let px = ((x1*y2-y1*x2)*(x3-x4)-(x1-x2)*(x3*y4-y3*x4)) / ((x1-x2)*(y3-y4)-(y1-y2)*(x3-x4));
  let py = ((x1*y2-y1*x2)*(y3-y4)-(y1-y2)*(x3*y4-y3*x4)) / ((x1-x2)*(y3-y4)-(y1-y2)*(x3-x4));

  (px, py)
}

fn contains(limits: (f64, f64), point: (f64, f64)) -> bool {
  point.0 >= limits.0 && point.0 <= limits.1 && point.1 >= limits.0 && point.1 <= limits.1
}

fn is_not_past(s: &Hailstone, p: (f64, f64)) -> bool {
 (p.0 - s.px) / s.vx >= 0f64 && (p.1 - s.py) / s.vy >= 0f64
}

fn part1(contents: &String) -> Result<usize, String> {
  let hailstones = match parse_hailstones(contents) {
    Ok(hailstones) => hailstones,
    Err(error) => return Err(error),
  };

  let limits = (200000000000000f64, 400000000000000f64);

  let mut passed = 0;

  for i in 0..hailstones.len() {
    for j in (i + 1)..hailstones.len() {
      let p = intersection(&hailstones[i], &hailstones[j]);

      if contains(limits, p) && is_not_past(&hailstones[i], p) && is_not_past(&hailstones[j], p) {
        passed += 1;
      }
    }
  }

  Ok(passed)
}

fn part2(_contents: &String) -> Result<usize, String> {
  Err("not implemented".into())
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