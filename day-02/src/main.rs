use std::fs;
use std::cmp;

fn part1(contents: &String) -> u32 {
  contents.lines()
    .map(|line| {
      let colon_pos = line.find(':').unwrap(); // [Game 1]:[ 4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red]

      let game_id = line["Game ".len()..colon_pos].parse::<u32>().unwrap(); // Game [1]: 4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red

      let has_valid_sets = line[(colon_pos + ": ".len())..] // Game 1: [4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red]
        .split("; ") // [4 blue, 16 green, 2 red]; [5 red, 11 blue, 16 green]; [9 green, 11 blue]; [10 blue, 6 green, 4 red]
        .all(|game_set| {
          game_set
            .split(", ") // [4 blue], [16 green], [2 red]
            .all(|set_result| {
              let mut parts = set_result.split(" "); // [4] [blue]

              let value = if let Some(value_str) = parts.next() { value_str.parse::<u32>().unwrap() } else { 999 };
              let color = if let Some(color_str) = parts.next() { color_str } else { "" };

              match color {
                "blue" => 14 >= value,
                "green" => 13 >= value,
                "red" => 12 >= value,
                _ => false,
              }
            })
        });

      if has_valid_sets {
        game_id
      } else {
        0
      }
    })
    .sum::<u32>()
}

fn part2(contents: &String) -> u32 {
  contents.lines()
    .map(|line| {
      let colon_pos = line.find(':').unwrap(); // [Game 1]:[ 4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red]

      struct CubeCounts {
        blue: u32,
        green: u32,
        red: u32,
      }

      let cube_counts = line[(colon_pos + ": ".len())..] // Game 1: [4 blue, 16 green, 2 red; 5 red, 11 blue, 16 green; 9 green, 11 blue; 10 blue, 6 green, 4 red]
        .split("; ") // [4 blue, 16 green, 2 red]; [5 red, 11 blue, 16 green]; [9 green, 11 blue]; [10 blue, 6 green, 4 red]
        .fold(CubeCounts{ blue: 0, green: 0, red: 0 }, |acc, game_set| {
          game_set
            .split(", ") // [4 blue], [16 green], [2 red]
            .fold(acc, |acc, set_result| {
              let mut parts = set_result.split(" "); // [4] [blue]

              let value = if let Some(value_str) = parts.next() { value_str.parse::<u32>().unwrap() } else { 999 };
              let color = if let Some(color_str) = parts.next() { color_str } else { "" };

              match color {
                "blue" => CubeCounts{
                  blue: cmp::max(acc.blue,value),
                  green: acc.green,
                  red: acc.red,
                },
                "green" => CubeCounts{
                  blue: acc.blue,
                  green: cmp::max(acc.green, value),
                  red: acc.red,
                },
                "red" => CubeCounts{
                  blue: acc.blue,
                  green: acc.green,
                  red: cmp::max(acc.red, value),
                },
                _ => CubeCounts{
                  blue: acc.blue,
                  green: acc.green,
                  red: acc.red,
                },
              }
            })
        });

      cube_counts.blue * cube_counts.green * cube_counts.red
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
