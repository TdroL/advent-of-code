use std::{fs, collections::HashMap};
#[derive(Copy, Clone)]
enum Op {
  Greater,
  Lesser,
}

struct RejectCondition {
  value_index: usize,
  compare_op: Op,
  test_value: usize,
}

struct AcceptCondition {
  value_index: usize,
  compare_op: Op,
  test_value: usize,
}

struct RedirectCondition<'a> {
  value_index: usize,
  compare_op: Op,
  test_value: usize,
  redirect_to: &'a str,
}

enum Rule<'a> {
  RejectCondition(RejectCondition),
  AcceptCondition(AcceptCondition),
  RedirectCondition(RedirectCondition<'a>),
  Reject,
  Accept,
  Redirect(&'a str),
}

struct Machine<'a> {
  workflows: Vec<(&'a str, Vec<Rule<'a>>)>,
  indices: HashMap<&'a str, usize>,
}

fn parse_machine<'a>(contents: &'a String) -> Result<Machine<'a>, String> {
  let mut workflows = vec![];

  for line in contents.lines() {
    if line.is_empty() || line.starts_with("{") {
      break;
    }

    let line = line.trim();

    let rule_start_pos = match line.find('{') {
      Some(rule_start_pos) => rule_start_pos,
      None => return Err(format!("unable to parse rule \"{line}\": missing \"{}\"", "{")),
    };
    let rule_end_pos = match line.find('}') {
      Some(rule_start_pos) => rule_start_pos,
      None => return Err(format!("unable to parse rule \"{line}\": missing \"{}\"", "}")),
    };

    let rule_name = &line[0..rule_start_pos];
    let mut rules = vec![];

    let mut conditions = line[(rule_start_pos + 1)..rule_end_pos].split(',');

    while let Some(condition) = conditions.next() {
      if condition == "A" {
        rules.push(Rule::Accept);
        continue;
      }

      if condition == "R" {
        rules.push(Rule::Reject);
        continue;
      }

      if let Some(colon_pos) = condition.find(':') {
        let op_pos = match condition.find(['<', '>']) {
          Some(op_pos) => op_pos,
          None => return Err(format!("unable to parse rule \"{line}\": invalid conditional rule \"{condition}\"")),
        };

        let compare_op = match &condition[op_pos..(op_pos + 1)] {
          ">" => Op::Greater,
          "<" => Op::Lesser,
          _ => return Err(format!("unable to parse workflow \"{line}\": invalid op \"{}\" in rule \"{condition}\"", &condition[op_pos..(op_pos + 1)])),
        };

        let test_value = match condition[(op_pos + 1)..colon_pos].parse::<usize>() {
          Ok(test_value) => test_value,
          Err(error) => return Err(format!("unable to parse rule \"{line}\": invalid value in rule \"{condition}\" - {error}")),
        };

        let value_index = match &condition[0..op_pos] {
          "x" => 0usize,
          "m" => 1usize,
          "a" => 2usize,
          "s" => 3usize,
          _ => return Err(format!("unable to parse workflow \"{line}\": invalid rating \"{}\" in rule \"{condition}\"", &condition[0..op_pos])),
        };

        let redirect_to = &condition[(colon_pos + 1)..];

        match redirect_to {
          "A" => {
            rules.push(Rule::AcceptCondition(AcceptCondition{ value_index, compare_op, test_value }));
          },
          "R" => {
            rules.push(Rule::RejectCondition(RejectCondition{ value_index, compare_op, test_value }));
          },
          _ => {
            rules.push(Rule::RedirectCondition(RedirectCondition{ value_index, compare_op, test_value, redirect_to }));
          },
        }

        continue;
      }

      rules.push(Rule::Redirect(condition));
    }

    workflows.push((rule_name, rules));
  }

  let mut indices = HashMap::new();

  for (index, &(rule_name, _)) in workflows.iter().enumerate() {
    indices.insert(rule_name, index);
  }


  Ok(Machine{ workflows, indices })
}

fn optimize_workflow_resolve_redirect_rules(machine: &mut Machine, index: usize) -> bool {
  for i in 0..machine.workflows[index].1.len() {
    match machine.workflows[index].1[i] {
      Rule::Redirect(target_rule_name) => {
        if let Some(&target_index) = machine.indices.get(target_rule_name) {
          if machine.workflows[target_index].1.len() == 1 {
            match machine.workflows[target_index].1[0] {
              Rule::Accept => {
                machine.workflows[index].1[i] = Rule::Accept;
              },
              Rule::Reject => {
                machine.workflows[index].1[i] = Rule::Reject;
              },
              _ => {},
            }
          }
        }
      },
      _ => {},
    }
  }

  return false;
}

fn optimize_workflow_resolve_redirect_condition_rules(machine: &mut Machine, index: usize) -> bool {
  for i in 0..machine.workflows[index].1.len() {
    match &machine.workflows[index].1[i] {
      Rule::RedirectCondition(redirect_condition) => {
        if let Some(&target_index) = machine.indices.get(redirect_condition.redirect_to) {
          if machine.workflows[target_index].1.len() == 1 {
            match machine.workflows[target_index].1[0] {
              Rule::Accept => {
                machine.workflows[index].1[i] = Rule::AcceptCondition(AcceptCondition{
                  value_index: redirect_condition.value_index,
                  compare_op: redirect_condition.compare_op,
                  test_value: redirect_condition.test_value,
                });
              },
              Rule::Reject => {
                machine.workflows[index].1[i] = Rule::RejectCondition(RejectCondition{
                  value_index: redirect_condition.value_index,
                  compare_op: redirect_condition.compare_op,
                  test_value: redirect_condition.test_value,
                });
              },
              _ => {},
            }
          }
        }
      },
      _ => {},
    }
  }

  return false;
}

fn optimize_workflow_with_only_accept_rules(machine: &mut Machine, index: usize) -> bool {
  let (_, rules) = &machine.workflows[index];

  if rules.len() <= 1 {
    return false;
  }

  let has_only_accept_rules = rules.iter().all(|rule| {
    match rule {
      Rule::AcceptCondition(_) | Rule::Accept => true,
      _ => false,
    }
  });

  if !has_only_accept_rules {
    return false;
  }

  machine.workflows[index].1.clear();
  machine.workflows[index].1.push(Rule::Accept);

  return true;
}

fn optimize_workflow_with_only_reject_rules(machine: &mut Machine, index: usize) -> bool {
  let (_, rules) = &machine.workflows[index];

  if rules.len() <= 1 {
    return false;
  }

  let has_only_reject_rules = rules.iter().all(|rule| {
    match rule {
      Rule::RejectCondition(_) | Rule::Reject => true,
      _ => false,
    }
  });

  if !has_only_reject_rules {
    return false;
  }

  machine.workflows[index].1.clear();
  machine.workflows[index].1.push(Rule::Reject);

  return true;
}

fn optimize_workflows<'a>(mut machine: Machine) -> Machine {
  let mut workflow_was_changed = true;

  while workflow_was_changed {
    workflow_was_changed = false;

    for index in 0..machine.workflows.len() {
      workflow_was_changed |= optimize_workflow_resolve_redirect_rules(&mut machine, index);
      workflow_was_changed |= optimize_workflow_resolve_redirect_condition_rules(&mut machine, index);
      workflow_was_changed |= optimize_workflow_with_only_accept_rules(&mut machine, index);
      workflow_was_changed |= optimize_workflow_with_only_reject_rules(&mut machine, index);
    }
  }

  machine
}

fn parse_parts(contents: &String) -> Result<Vec<[usize; 4]>, String> {
  let mut parts = vec![];

  for line in contents.lines() {
    if line.is_empty() || !line.starts_with("{") || !line.ends_with("}") {
      continue;
    }

    let mut part_definitions = line[1..(line.len() - 1)].split(',');

    parts.push([0, 0, 0, 0]);
    let part_sets = match parts.last_mut() {
      Some(workflow) => workflow,
      None => return Err(format!("unexpected error")),
    };

    while let Some(part_definition) = part_definitions.next() {
      let mid = match part_definition.find('=') {
        Some(mid) => mid,
        None => return Err(format!("unable to parse part set \"{line}\": missing \"=\" in \"{part_definition}\"")),
      };

      let (rating, value) = part_definition.split_at(mid);

      let index = match rating {
        "x" => 0usize,
        "m" => 1usize,
        "a" => 2usize,
        "s" => 3usize,
        _ => return Err(format!("unable to parse part set \"{line}\": invalid rating \"{rating}\"")),
      };

      let value = match value[1..].parse::<usize>() {
        Ok(value) => value,
        Err(error) => return Err(format!("unable to parse part set \"{line}\": {error}")),
      };

      part_sets[index] = value;
    }
  }

  Ok(parts)
}


fn process_part_set(machine: &Machine, part_set: &[usize; 4]) -> Result<bool, String> {
  let mut workflow_index = match machine.indices.get("in") {
    Some(&index) => index,
    None => return Err(format!("unable to find workflow \"in\"")),
  };

  loop {
    let rules = &machine.workflows[workflow_index].1;

    'rules_loop: for rule in rules {
      match rule {
        Rule::RejectCondition(reject_condition) => {
          match reject_condition.compare_op {
            Op::Greater => {
              if part_set[reject_condition.value_index] > reject_condition.test_value {
                return Ok(false);
              }
            },
            Op::Lesser => {
              if part_set[reject_condition.value_index] < reject_condition.test_value {
                return Ok(false);
              }
            },
          }
        },
        Rule::AcceptCondition(accept_condition) => {
          match accept_condition.compare_op {
            Op::Greater => {
              if part_set[accept_condition.value_index] > accept_condition.test_value {
                return Ok(true);
              }
            },
            Op::Lesser => {
              if part_set[accept_condition.value_index] < accept_condition.test_value {
                return Ok(true);
              }
            },
          }
        },
        Rule::RedirectCondition(redirect_condition) => {
          let should_redirect = match redirect_condition.compare_op {
            Op::Greater => part_set[redirect_condition.value_index] > redirect_condition.test_value,
            Op::Lesser => part_set[redirect_condition.value_index] < redirect_condition.test_value,
          };

          if should_redirect {
            workflow_index = match machine.indices.get(redirect_condition.redirect_to) {
              Some(&index) => index,
              None => return Err(format!("unable to find workflow \"{workflow_index}\"")),
            };

            break 'rules_loop;
          }
        },
        Rule::Reject => {
          return Ok(false);
        },
        Rule::Accept => {
          return Ok(true);
        },
        Rule::Redirect(workflow_name) => {
          workflow_index = match machine.indices.get(workflow_name) {
            Some(&index) => index,
            None => return Err(format!("unable to find workflow \"{workflow_index}\"")),
          };

          break 'rules_loop;
        },
      }
    }
  }
}


#[derive(Copy, Clone)]
struct Range {
  from: usize,
  to: usize,
}

fn split_ranges(ranges: &[Range; 4], index: usize, value: usize) -> ([Range; 4], [Range; 4]) {
  let mut left = ranges.clone();
  let mut right = ranges.clone();

  if left[index].to > value {
    left[index].to = value;

    if right[index].from < value {
      right[index].from = value + 1;
    }
  } else {
    right[index].from = right[index].to + 1;
  }

  (left, right)
}

fn sum_ranges(ranges: &[Range; 4]) -> usize {
  (ranges[0].to - ranges[0].from + 1) * (ranges[1].to - ranges[1].from + 1) * (ranges[2].to - ranges[2].from + 1) * (ranges[3].to - ranges[3].from + 1)
}

fn find_possible_distinct_combinations<'a>(machine: &Machine<'a>, workflow_index: usize, ranges: [Range; 4]) -> Result<usize, String> {
  let rules = &machine.workflows[workflow_index].1;

  let mut ranges = ranges;

  let mut sum = 0;

  for rule in rules {
    match rule {
      Rule::RejectCondition(reject_condition) => {
        let left_ranges = match reject_condition.compare_op {
          Op::Greater => {
            let (left, _) = split_ranges(&ranges, reject_condition.value_index, reject_condition.test_value);

            left
          },
          Op::Lesser => {
            let (_, right) = split_ranges(&ranges, reject_condition.value_index, reject_condition.test_value - 1);

            right
          },
        };

        ranges = left_ranges;
      },
      Rule::AcceptCondition(accept_condition) => {
        let (accepted_ranges, left_ranges) = match accept_condition.compare_op {
          Op::Greater => {
            let (left, right) = split_ranges(&ranges, accept_condition.value_index, accept_condition.test_value);

            (right, left)
          },
          Op::Lesser => {
            let (left, right) = split_ranges(&ranges, accept_condition.value_index, accept_condition.test_value - 1);

            (left, right)
          },
        };

        sum = sum + sum_ranges(&accepted_ranges);
        ranges = left_ranges;
      },
      Rule::RedirectCondition(redirect_condition) => {
        let (redirected_ranges, left_ranges) = match redirect_condition.compare_op {
          Op::Greater => {
            let (left, right) = split_ranges(&ranges, redirect_condition.value_index, redirect_condition.test_value);

            (right, left)
          },
          Op::Lesser => {
            let (left, right) = split_ranges(&ranges, redirect_condition.value_index, redirect_condition.test_value - 1);

            (left, right)
          },
        };

        let partial_sum = match machine.indices.get(redirect_condition.redirect_to) {
          Some(&next_workflow_index) => match find_possible_distinct_combinations(&machine, next_workflow_index, redirected_ranges) {
            Ok(partial_sum) => partial_sum,
            Err(error) => return Err(error),
          },
          None => return Err(format!("unable to find workflow \"{workflow_index}\"")),
        };

        sum = sum + partial_sum;
        ranges = left_ranges;
      },
      Rule::Reject => {
        return Ok(sum);
      },
      Rule::Accept => {
        return Ok(sum + sum_ranges(&ranges));
      },
      Rule::Redirect(workflow_name) => {
        match machine.indices.get(workflow_name) {
          Some(&next_workflow_index) => match find_possible_distinct_combinations(&machine, next_workflow_index, ranges) {
            Ok(partial_sum) => return Ok(sum + partial_sum),
            Err(error) => return Err(error),
          },
          None => return Err(format!("unable to find workflow \"{workflow_index}\"")),
        }
      },
    }
  }

  Ok(sum)
}

fn part1(contents: &String) -> Result<usize, String> {
  let machine = match parse_machine(contents) {
    Ok(machine) => machine,
    Err(error) => return Err(error),
  };
  let parts = match parse_parts(contents) {
    Ok(parts) => parts,
    Err(error) => return Err(error),
  };

  let machine = optimize_workflows(machine);

  let processed_parts = parts
    .iter()
    .map(|part_set| {
      match process_part_set(&machine, part_set) {
        Ok(result) => Ok((part_set, result)),
        Err(error) => Err(error),
      }
    })
    .into_iter()
    .collect::<Result<Vec<(&[usize; 4], bool)>, String>>();

  let processed_parts = match processed_parts {
    Ok(processed_parts) => processed_parts,
    Err(error) => return Err(error),
  };

  let sum = processed_parts
    .iter()
    .filter(|(_, result)| *result)
    .map(|(part_set, _)| part_set[0] + part_set[1] + part_set[2] + part_set[3])
    .sum::<usize>();

  Ok(sum)
}

fn part2(contents: &String) -> Result<usize, String> {
  let machine = match parse_machine(contents) {
    Ok(machine) => machine,
    Err(error) => return Err(error),
  };

  let machine = optimize_workflows(machine);

  let workflow_index = match machine.indices.get("in") {
    Some(&workflow_index) => workflow_index,
    None => return Err(format!("unable to find workflow \"in\"")),
  };

  find_possible_distinct_combinations(&machine, workflow_index, [Range{ from: 1, to: 4000 }, Range{ from: 1, to: 4000 }, Range{ from: 1, to: 4000 }, Range{ from: 1, to: 4000 }])
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