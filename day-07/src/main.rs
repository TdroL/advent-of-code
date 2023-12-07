use std::{fs, cmp::Ordering};

fn parse_line(line: &str) -> (&str, u32) {
  let mut parts = line.split_whitespace();

  let hand = parts.next().unwrap();
  let bid = parts.next().unwrap().parse::<u32>().unwrap();

  (hand, bid)
}

fn card_strength(card: char) -> u32 {
  match card {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 11,
    'T' => 10,
    '9' => 9,
    '8' => 8,
    '7' => 7,
    '6' => 6,
    '5' => 5,
    '4' => 4,
    '3' => 3,
    '2' => 2,
    _ => 0,
  }
}

fn card_strength_with_jokers(card: char) -> u32 {
  match card {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'T' => 10,
    '9' => 9,
    '8' => 8,
    '7' => 7,
    '6' => 6,
    '5' => 5,
    '4' => 4,
    '3' => 3,
    '2' => 2,
    'J' => 1,
    _ => 0,
  }
}

fn length_of_equal(hand_sorted: &[u32]) -> usize {
  if hand_sorted.len() == 0 {
    return 0;
  }

  let mut i = 1;
  while i < hand_sorted.len() {
    if hand_sorted[0] != hand_sorted[i] {
      return i;
    }

    i = i + 1;
  }

  i
}

fn length_of_jokers(hand_sorted: &[u32; 5]) -> usize {
  const JOKER: u32 = 1;

  let mut i = 0;

  while i < hand_sorted.len() && hand_sorted[hand_sorted.len() - 1 - i] == JOKER {
    i = i + 1;
  }
  i
}

fn hand_strength(hand_sorted: &[u32; 5]) -> u32 {
  let lengths_of_equal = [
    length_of_equal(&hand_sorted[0..]),
    length_of_equal(&hand_sorted[1..]),
    length_of_equal(&hand_sorted[2..]),
    length_of_equal(&hand_sorted[3..]),
    length_of_equal(&hand_sorted[4..]),
  ];

  /*
  XXXXX
  */
  if lengths_of_equal[0] == 5 {
    return 6;
  }

  /*
  XXXXY
  YXXXX
  */
  if lengths_of_equal[0] == 4 || lengths_of_equal[1] == 4 {
    return 5;
  }

  /*
  XXXYY
  YYXXX
  */
  if (lengths_of_equal[0] == 3 && lengths_of_equal[3] == 2) || (lengths_of_equal[0] == 2 && lengths_of_equal[2] == 3) {
    return 4;
  }

  /*
  XXXYZ
  YXXXZ
  YZXXX
  */
  if lengths_of_equal[0] == 3 || lengths_of_equal[1] == 3 || lengths_of_equal[2] == 3 {
    return 3;
  }

  /*
  XXYYZ
  XXZYY
  ZXXYY
  */
  if (lengths_of_equal[0] == 2 && lengths_of_equal[2] == 2) || (lengths_of_equal[0] == 2 && lengths_of_equal[3] == 2) || (lengths_of_equal[1] == 2 && lengths_of_equal[3] == 2) {
    return 2;
  }

  /*
  XXYZW
  YXXZW
  YZXXW
  YZWXX
  */
  if lengths_of_equal[0] == 2 || lengths_of_equal[1] == 2 || lengths_of_equal[2] == 2 || lengths_of_equal[3] == 2 {
    return 1;
  }

  0
}

fn hand_strength_with_jokers(hand_sorted: &[u32; 5]) -> u32 {
  let jokers = length_of_jokers(&hand_sorted);

  let lengths_of_equal = [
    if jokers < 5 { length_of_equal(&hand_sorted[0..]) } else { 0 },
    if jokers < 4 { length_of_equal(&hand_sorted[1..]) } else { 0 },
    if jokers < 3 { length_of_equal(&hand_sorted[2..]) } else { 0 },
    if jokers < 2 { length_of_equal(&hand_sorted[3..]) } else { 0 },
    if jokers < 1 { length_of_equal(&hand_sorted[4..]) } else { 0 },
  ];

  /*
  XXXXX

  XXXXJ

  XXXJJ

  XXJJJ

  XJJJJ

  JJJJJ
  */
  if lengths_of_equal[0] + jokers == 5 {
    return 6;
  }

  /*
  XXXXY
  YXXXX

  XXXYJ
  YXXXJ

  XXYJJ
  YXXJJ

  XYJJJ
  YXJJJ

  YJJJJ
  */
  if lengths_of_equal[0] + jokers == 4 || lengths_of_equal[1] + jokers == 4 {
    return 5;
  }

  /*
  XXXYY
  YYXXX

  XXYYJ
  YYXXJ
  */
  if (lengths_of_equal[0] == 3 && lengths_of_equal[3] == 2) || (lengths_of_equal[0] == 2 && lengths_of_equal[2] == 3) ||
     (lengths_of_equal[0] == 2 && lengths_of_equal[2] == 2 && jokers == 1) {
    return 4;
  }

  /*
  XXXYZ
  YXXXZ
  YZXXX

  XXYZJ
  YXXZJ
  YZXXJ

  XYZJJ
  */
  if lengths_of_equal[0] + jokers == 3 || lengths_of_equal[1] + jokers == 3 || lengths_of_equal[2] + jokers == 3 {
    return 3;
  }

  /*
  XXYYZ
  XXZYY
  ZXXYY
  */
  if (lengths_of_equal[0] == 2 && lengths_of_equal[2] == 2) || (lengths_of_equal[0] == 2 && lengths_of_equal[3] == 2) || (lengths_of_equal[1] == 2 && lengths_of_equal[3] == 2) {
    return 2;
  }

  /*
  XXYZW
  YXXZW
  YZXXW
  YZWXX

  XYZWJ
  */
  if lengths_of_equal[0] + jokers == 2 || lengths_of_equal[1] + jokers == 2 || lengths_of_equal[2] + jokers == 2 || lengths_of_equal[3] + jokers == 2 {
    return 1;
  }

  0
}

fn resolve_hand(hand: &str) -> ([u32; 5], u32) {
  let mut chars = hand.chars();

  let mut hand_resolved = [1, 1, 1, 1, 1];
  let mut hand_sorted = [1, 1, 1, 1, 1];
  for i in 0..5 {
    if let Some(card) = chars.next() {
      hand_resolved[i] = card_strength(card);
      hand_sorted[i] = hand_resolved[i];
    } else {
      break;
    }
  }

  hand_sorted.sort_by(|a, b| b.cmp(a));

  (hand_resolved, hand_strength(&hand_sorted))
}

fn resolve_hand_with_jokers(hand: &str) -> ([u32; 5], u32) {
  let mut chars = hand.chars();

  let mut hand_resolved = [1, 1, 1, 1, 1];
  let mut hand_sorted = [1, 1, 1, 1, 1];
  for i in 0..5 {
    if let Some(card) = chars.next() {
      hand_resolved[i] = card_strength_with_jokers(card);
      hand_sorted[i] = hand_resolved[i];
    } else {
      break;
    }
  }

  hand_sorted.sort_by(|a, b| b.cmp(a));

  (hand_resolved, hand_strength_with_jokers(&hand_sorted))
}

fn part1(contents: &String) -> u32 {
  let mut hands_with_strength_and_bids = contents.lines()
    .map(|line| {
      let (hand, bid) = parse_line(line);
      let (hand_resolved, hand_strength) = resolve_hand(hand);

      (hand_resolved, hand_strength, bid)
    })
    .collect::<Vec<([u32; 5], u32, u32)>>();

  hands_with_strength_and_bids.sort_by(|a, b| {
    let (a_hand, a_strength, _) = a;
    let (b_hand, b_strength, _) = b;

    let strength_ord = a_strength.cmp(b_strength);
    if strength_ord != Ordering::Equal {
      return strength_ord;
    }

    for i in 0..5 {
      let hand_ord = a_hand[i].cmp(&b_hand[i]);
      if hand_ord != Ordering::Equal {
        return hand_ord;
      }
    }

    Ordering::Equal
  });

  hands_with_strength_and_bids
    .iter()
    .enumerate()
    .map(|(index, (_, _, bid))| (index as u32 + 1) * bid)
    .sum::<u32>()
}

fn part2(contents: &String) -> u32 {
  let mut hands_with_strength_and_bids = contents.lines()
    .map(|line| {
      let (hand, bid) = parse_line(line);
      let (hand_resolved, hand_strength) = resolve_hand_with_jokers(hand);

      (hand_resolved, hand_strength, bid)
    })
    .collect::<Vec<([u32; 5], u32, u32)>>();

  hands_with_strength_and_bids.sort_by(|a, b| {
    let (a_hand, a_strength, _) = a;
    let (b_hand, b_strength, _) = b;

    let strength_ord = a_strength.cmp(b_strength);
    if strength_ord != Ordering::Equal {
      return strength_ord;
    }

    for i in 0..5 {
      let hand_ord = a_hand[i].cmp(&b_hand[i]);
      if hand_ord != Ordering::Equal {
        return hand_ord;
      }
    }

    Ordering::Equal
  });

  hands_with_strength_and_bids
    .iter()
    .enumerate()
    .map(|(index, (_, _, bid))| (index as u32 + 1) * bid)
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
