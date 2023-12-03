use std::fs;

#[derive(Copy, Clone)]
struct NumberPositioning {
  value: u32,
  start: usize,
  end: usize,
}

fn extract_all_numbers_in_line(line: &str) -> Vec<NumberPositioning> {
  let mut is_consuming_tokens = false;
  let mut start = 0;

  let mut result = Vec::new();

  for (index, symbol) in line.chars().enumerate() {
    if symbol.is_ascii_digit() {
      if !is_consuming_tokens {
        start = index;
        is_consuming_tokens = true;
      }
    } else {
      if is_consuming_tokens {
        let end = index;
        let value = line[start..end].parse::<u32>().unwrap();

        is_consuming_tokens = false;

        result.push(NumberPositioning{ value, start, end });
      }
    }
  }

  // end-of-line
  if is_consuming_tokens {
    let end = line.len();
    let value = line[start..end].parse::<u32>().unwrap();

    result.push(NumberPositioning{ value, start, end });
  }

  result
}

struct SymbolPositioning {
  symbol: char,
  index: usize,
}

fn extract_all_symbols_in_line(line: &str) -> Vec<SymbolPositioning> {
  let mut result = Vec::new();

  for (index, symbol) in line.chars().enumerate() {
    if !symbol.is_ascii_digit() && symbol != '.' {
      result.push(SymbolPositioning{ symbol, index });
    }
  }

  result
}

fn has_row_adjacent_symbol(row: usize, number_positioning: &NumberPositioning, symbols_in_lines: &Vec<Vec<SymbolPositioning>>) -> bool {
  if let Some(symbols_in_line) = symbols_in_lines.get(row) {
    let number_padded_start = if number_positioning.start > 0 { number_positioning.start - 1 } else { number_positioning.start };
    let number_padded_end = number_positioning.end + 1;

    for symbol_positioning in symbols_in_line {
      let symbol_index = symbol_positioning.index;

      if number_padded_start <= symbol_index && symbol_index < number_padded_end {
        return true;
      }
    }
  }

  false
}

fn filter_row_adjacent_numbers(row: usize, symbol_positioning: &SymbolPositioning, numbers_in_lines: &Vec<Vec<NumberPositioning>>) -> Vec<NumberPositioning> {
  if let Some(numbers_in_line) = numbers_in_lines.get(row) {
    let symbol_index = symbol_positioning.index;

    return numbers_in_line
      .iter()
      .filter(|number_positioning| {
        let number_padded_start = if number_positioning.start > 0 { number_positioning.start - 1 } else { number_positioning.start };
        let number_padded_end = number_positioning.end + 1;

        number_padded_start <= symbol_index && symbol_index < number_padded_end
      })
      .map(|number_positioning| *number_positioning)
      .collect();
  }

  Vec::new()
}

fn part1(contents: &String) -> u32 {
  let symbols_in_lines = contents
    .lines()
    .map(|line| extract_all_symbols_in_line(line))
    .collect::<Vec<Vec<SymbolPositioning>>>();

  contents
    .lines()
    .enumerate()
    .map(|(row, line)| {
      extract_all_numbers_in_line(line)
        .iter()
        .map(|number_positioning| {
          if row > 0 && has_row_adjacent_symbol(row - 1, number_positioning, &symbols_in_lines) {
            return number_positioning.value;
          }

          if has_row_adjacent_symbol(row, number_positioning, &symbols_in_lines) {
            return number_positioning.value;
          }

          if row + 1 < symbols_in_lines.len() && has_row_adjacent_symbol(row + 1, number_positioning, &symbols_in_lines) {
            return number_positioning.value;
          }

          0
        })
        .sum::<u32>()
    })
    .sum::<u32>()
}

fn part2(contents: &String) -> u32 {
  let numbers_in_lines = contents
    .lines()
    .map(|line| extract_all_numbers_in_line(line))
    .collect::<Vec<Vec<NumberPositioning>>>();

  contents
    .lines()
    .enumerate()
    .map(|(row, line)| {
      extract_all_symbols_in_line(line)
        .iter()
        .map(|symbol_positioning| {
          if symbol_positioning.symbol != '*' {
            return 0;
          }

          let mut adjacent_numbers = Vec::new();

          if row > 0 {
            adjacent_numbers.append(&mut filter_row_adjacent_numbers(row - 1, symbol_positioning, &numbers_in_lines));
          }

          adjacent_numbers.append(&mut filter_row_adjacent_numbers(row, symbol_positioning, &numbers_in_lines));

          if row + 1 < numbers_in_lines.len() {
            adjacent_numbers.append(&mut filter_row_adjacent_numbers(row + 1, symbol_positioning, &numbers_in_lines));
          }

          if adjacent_numbers.len() != 2 {
            return 0;
          }

          adjacent_numbers
            .iter()
            .fold(1, |acc, number_positioning| acc * number_positioning.value)
        })
        .sum::<u32>()
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
