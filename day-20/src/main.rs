use std::{fs, collections::{HashMap, VecDeque, HashSet}};

#[derive(Clone)]
struct BroadcasterModule<'a> {
  name: &'a str,
  outputs: Vec<&'a str>,
}

#[derive(Clone)]
struct FlipFlowModule<'a> {
  name: &'a str,
  outputs: Vec<&'a str>,
  enabled: bool,
}

#[derive(Clone)]
struct ConjunctionModule<'a> {
  name: &'a str,
  remembered_signals: HashMap<&'a str, bool>,
  outputs: Vec<&'a str>,
}

#[derive(Clone)]
enum Module<'a> {
  BroadcasterModule(BroadcasterModule<'a>),
  FlipFlowModule(FlipFlowModule<'a>),
  ConjunctionModule(ConjunctionModule<'a>),
}

fn consume_whitespace(source: &str) -> &str {
  source.trim_start()
}

fn consume_symbol(source: &str) -> (&str, &str) {
  let source = consume_whitespace(source);

  if let Some(pos) = source.find(|c: char| c.is_whitespace() || c.is_alphanumeric()) {
    source.split_at(pos)
  } else {
    (source, &source[0..0])
  }
}

fn consume_identifier(source: &str) -> (&str, &str) {
  let source = consume_whitespace(source);

  if let Some(pos) = source.find(|c: char| !c.is_alphanumeric()) {
    source.split_at(pos)
  } else {
    (source, &source[0..0])
  }
}

fn parse_comma_separated_identifiers(source: &str) -> Result<Vec<&str>, String> {
  let mut outputs = vec![];

  let mut parsed_source = source;
  loop {
    let source = parsed_source;
    let source = consume_whitespace(source);

    if source.is_empty() {
      break;
    }

    let (identifier, source) = consume_identifier(source);
    if identifier.is_empty() {
      return Err(format!("invalid identifier \"{source}\", expected output name"));
    }

    outputs.push(identifier);

    let (symbol, source) = consume_symbol(source);
    if symbol.is_empty() && source.is_empty() {
      // EOL
      break;
    }

    if symbol != "," {
      return Err(format!("invalid symbol \"{symbol}\", expected \",\""));
    }

    parsed_source = source;
  }

  Ok(outputs)
}

fn parse_broadcaster_module<'a>(line: &'a str) -> Result<Module<'a>, String> {
  let source = &line;

  let (name, source) = consume_identifier(source);
  if name.is_empty() {
    return Err(format!("unable to parse line \"{line}\": invalid identifier \"{source}\", expected broadcaster name"));
  }

  let (symbol, source) = consume_symbol(source);
  if symbol != "->" {
    return Err(format!("unable to parse line \"{line}\": invalid symbol \"{symbol}\", expected \"->\""));
  }

  let outputs = match parse_comma_separated_identifiers(source) {
    Ok(outputs) => outputs,
    Err(error) => return Err(format!("unable to parse line \"{line}\": {error}")),
  };

  Ok(Module::BroadcasterModule(BroadcasterModule{ name, outputs }))
}

fn parse_flip_flow_module<'a>(line: &'a str) -> Result<Module<'a>, String> {
  let source = &line;

  let (symbol, source) = consume_symbol(source);
  if symbol != "%" {
    return Err(format!("unable to parse line \"{line}\": invalid symbol \"{symbol}\", expected \"%\""));
  }

  let (name, source) = consume_identifier(source);
  if name.is_empty() {
    return Err(format!("unable to parse line \"{line}\": invalid identifier \"{source}\", expected flip-flop name"));
  }

  let (symbol, source) = consume_symbol(source);
  if symbol != "->" {
    return Err(format!("unable to parse line \"{line}\": invalid symbol \"{symbol}\", expected \"->\""));
  }

  let outputs = match parse_comma_separated_identifiers(source) {
    Ok(outputs) => outputs,
    Err(error) => return Err(format!("unable to parse line \"{line}\": {error}")),
  };

  Ok(Module::FlipFlowModule(FlipFlowModule{ name, outputs, enabled: false }))
}

fn parse_conjunction_module<'a>(line: &'a str) -> Result<Module<'a>, String> {
  let source = &line;

  let (symbol, source) = consume_symbol(source);
  if symbol != "&" {
    return Err(format!("unable to parse line \"{line}\": invalid symbol \"{symbol}\", expected \"&\""));
  }

  let (name, source) = consume_identifier(source);
  if name.is_empty() {
    return Err(format!("unable to parse line \"{line}\": invalid identifier \"{source}\", expected conjunction name"));
  }

  let (symbol, source) = consume_symbol(source);
  if symbol != "->" {
    return Err(format!("unable to parse line \"{line}\": invalid symbol \"{symbol}\", expected \"->\""));
  }

  let outputs = match parse_comma_separated_identifiers(source) {
    Ok(outputs) => outputs,
    Err(error) => return Err(format!("unable to parse line \"{line}\": {error}")),
  };

  Ok(Module::ConjunctionModule(ConjunctionModule{ name, remembered_signals: HashMap::new(), outputs }))
}

#[derive(Clone)]
struct Processor<'a> {
  modules: Vec<Module<'a>>,
  indices: HashMap<&'a str, usize>,
}

impl<'a> Processor<'a> {
  fn reset(&mut self) {
    for index in 0..self.modules.len() {
      match &mut self.modules[index] {
        Module::FlipFlowModule(module) => {
          module.enabled = false;
        },
        Module::ConjunctionModule(module) => {
          for entry in module.remembered_signals.iter_mut() {
            *entry.1 = false;
          }
        },
        _ => {},
      }
    }
  }
}

fn index_modules<'a>(modules: &Vec<Module<'a>>) -> HashMap<&'a str, usize> {
  let mut indices = HashMap::new();

  for index in 0..modules.len() {
    match &modules[index] {
      Module::BroadcasterModule(module) => {
        indices.insert(module.name, index);
      },
      Module::FlipFlowModule(module) => {
        indices.insert(module.name, index);
      },
      Module::ConjunctionModule(module) => {
        indices.insert(module.name, index);
      },
    }
  }

  indices
}

fn connect_conjunction_modules<'a>(mut modules: Vec<Module<'a>>, indices: &HashMap<&str, usize>) -> Vec<Module<'a>> {
  let mut mappings = vec![];

  for index in 0..modules.len() {
    match &modules[index] {
      Module::BroadcasterModule(module) => {
        for output in module.outputs.iter() {
          if let Some(&output_index) = indices.get(output) {
            mappings.push((module.name, output_index));
          }
        }
      },
      Module::FlipFlowModule(module) => {
        for output in module.outputs.iter() {
          if let Some(&output_index) = indices.get(output) {
            mappings.push((module.name, output_index));
          }
        }
      },
      Module::ConjunctionModule(module) => {
        for output in module.outputs.iter() {
          if let Some(&output_index) = indices.get(output) {
            mappings.push((module.name, output_index));
          }
        }
      },
    }
  }

  for (input_name,  output_index) in mappings {
    match &mut modules[output_index] {
      Module::ConjunctionModule(module) => {
        module.remembered_signals.insert(input_name, false);
      },
      _ => {},
    }
  }

  modules
}

fn parse_modules<'a>(contents: &'a String) -> Result<Processor<'a>, String> {
  let modules = contents
    .lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .map(|line| {
      if line.starts_with("%") {
        return parse_flip_flow_module(line);
      }

      if line.starts_with("&") {
        return parse_conjunction_module(line);
      }

      return parse_broadcaster_module(line);
    })
    .into_iter()
    .collect();

  let modules = match modules {
    Ok(modules) => modules,
    Err(error) => return Err(error),
  };

  let indices = index_modules(&modules);

  let modules = connect_conjunction_modules(modules, &indices);

  Ok(Processor{
    modules,
    indices,
  })
}

trait ProcessSignal<'a> {
  fn process(&mut self, signal: (&'a str, &'a str, bool)) -> Vec<(&'a str, &'a str, bool)>;
}

impl<'a> ProcessSignal<'a> for BroadcasterModule<'a> {
  fn process(&mut self, (_, _, is_high): (&'a str, &'a str, bool)) -> Vec<(&'a str, &'a str, bool)> {
    self.outputs
      .iter()
      .map(|&output_name| (self.name, output_name, is_high))
      .collect()
  }
}

impl<'a> ProcessSignal<'a> for FlipFlowModule<'a> {
  fn process(&mut self, (_, _, is_high): (&'a str, &'a str, bool)) -> Vec<(&'a str, &'a str, bool)> {
    if is_high {
      return vec![];
    }

    self.enabled = !self.enabled;

    self.outputs
      .iter()
      .map(|&output_name| (self.name, output_name, self.enabled))
      .collect()
  }
}

impl<'a> ProcessSignal<'a> for ConjunctionModule<'a> {
  fn process(&mut self, (source_name, _, is_high): (&'a str, &'a str, bool)) -> Vec<(&'a str, &'a str, bool)> {
    self.remembered_signals.insert(source_name, is_high);

    let has_only_high = self.remembered_signals
      .iter()
      .all(|(_, &is_high)| is_high);

    self.outputs
      .iter()
      .map(|&output_name| (self.name, output_name, !has_only_high))
      .collect()
  }
}

fn process_signal<'a>(processor: &mut Processor<'a>, initial_signal: (&'a str, &'a str, bool)) -> (usize, usize) {
  let mut signals = VecDeque::new();
  signals.push_back(initial_signal);

  let mut high_signal_count = 0;
  let mut low_signal_count = 0;
  while let Some(signal) = signals.pop_front() {
    let (_, target_name, is_high) = signal;

    if is_high {
      high_signal_count += 1;
    } else {
      low_signal_count += 1;
    }

    if let Some(&module_index) = processor.indices.get(target_name) {
      let next_signals = match &mut processor.modules[module_index] {
        Module::BroadcasterModule(module) => module.process(signal),
        Module::FlipFlowModule(module) => module.process(signal),
        Module::ConjunctionModule(module) => module.process(signal),
      };

      for next_signal in next_signals {
        signals.push_back(next_signal);
      }
    }
  }

  (high_signal_count, low_signal_count)
}

fn build_inverse_module_dependencies<'a>(processor: &'a Processor) -> HashMap<&'a str, Vec<&'a str>> {
  let mut inverse_dependencies = HashMap::new();

  let modules = &processor.modules;

  for index in 0..modules.len() {
    match &modules[index] {
      Module::BroadcasterModule(module) => {
        for &output in module.outputs.iter() {
          let dependencies = inverse_dependencies.entry(output).or_insert_with(|| vec![]);
          dependencies.push(module.name);
        }
      },
      Module::FlipFlowModule(module) => {
        for output in module.outputs.iter() {
          let dependencies = inverse_dependencies.entry(output).or_insert_with(|| vec![]);
          dependencies.push(module.name);
        }
      },
      Module::ConjunctionModule(module) => {
        for output in module.outputs.iter() {
          let dependencies = inverse_dependencies.entry(output).or_insert_with(|| vec![]);
          dependencies.push(module.name);
        }
      },
    }
  }

  inverse_dependencies
}

fn find_active_modules<'a>(inverse_dependencies: &HashMap<&'a str, Vec<&'a str>>, start: &'a str, end: &'a str) -> HashSet<&'a str> {
  let mut set = HashSet::new();
  set.insert(end);

  let mut queue = VecDeque::new();
  queue.push_back(start);

  while let Some(module_name) = queue.pop_front() {
    if set.contains(module_name) {
      continue;
    }

    set.insert(module_name);

    if let Some(dependencies) = inverse_dependencies.get(module_name) {
      for &dependency in dependencies {
        queue.push_back(dependency);
      }
    }
  }

  set
}

fn count_impulses_for_subsection<'a>(processor: &mut Processor<'a>, active_modules: HashSet<&str>, initial_signals: Vec<(&'a str, &'a str, bool)>, expected_signal: (&str, bool)) -> usize {
  processor.reset();

  let mut counter = 0;
  loop {
    counter += 1;

    let mut signals = VecDeque::new();
    for initial_signal in initial_signals.clone() {
      signals.push_back(initial_signal);
    }

    while let Some(signal) = signals.pop_front() {
      let (_, target_name, is_high) = signal;

      if target_name == expected_signal.0 && is_high == expected_signal.1 {
        return counter;
      }

      if let Some(&module_index) = processor.indices.get(target_name) {
        let next_signals = match &mut processor.modules[module_index] {
          Module::BroadcasterModule(module) => module.process(signal),
          Module::FlipFlowModule(module) => module.process(signal),
          Module::ConjunctionModule(module) => module.process(signal),
        };

        for next_signal in next_signals {
          if active_modules.contains(next_signal.1) {
            signals.push_back(next_signal);
          }
        }
      }
    }
  }
}

fn gcd(a: usize, b: usize) -> usize {
  let mut a = a;
  let mut b = b;

  while a != b {
    if a > b {
      a = a - b;
    } else {
      b = b - a;
    }
  }

  a
}

fn lcm(a: usize, b: usize) -> usize {
  (a * b) / gcd(a, b)
}

fn find_minimum_impluses(processor: &Processor, from: &str, to: &str) -> Result<usize, String> {
  let inverse_dependencies = build_inverse_module_dependencies(&processor);

  let mut candidate = to;
  let mut expected_signal = false;

  loop {
    let dependencies = match inverse_dependencies.get(candidate) {
      Some(dependencies) => dependencies,
      None => return Err(format!("unable to find parents of \"{candidate}\"")),
    };

    if dependencies.len() == 0 {
      return Err(format!("unable to find dependencies of \"{candidate}\""));
    } else if dependencies.len() == 1 {
      candidate = dependencies[0];

      let module_index = match processor.indices.get(candidate) {
        Some(&module_index) => module_index,
        None => return Err(format!("unable to find index of \"{candidate}\"")),
      };

      expected_signal = match &processor.modules[module_index] {
        Module::BroadcasterModule(_) => expected_signal,
        Module::FlipFlowModule(_) => false,
        Module::ConjunctionModule(_) => true,
      };
    } else {
      let mut required_dependency_impulses = 1;

      for &dependency in dependencies.iter() {
        let mut active_modules = find_active_modules(&inverse_dependencies, dependency, from);
        active_modules.insert(candidate);

        let impulses = count_impulses_for_subsection(&mut processor.clone(), active_modules, vec![("button", from, false)], (candidate, expected_signal));

        required_dependency_impulses = lcm(required_dependency_impulses, impulses);
      }

      let head_initial_impulses = dependencies
        .iter()
        .map(|&dependency| (dependency, candidate, expected_signal))
        .collect::<Vec<(&str, &str, bool)>>();

      let head_active_modules = find_active_modules(&inverse_dependencies, to, candidate);

      let head_impulses = count_impulses_for_subsection(&mut processor.clone(), head_active_modules, head_initial_impulses, (to, false));

      return Ok(required_dependency_impulses * head_impulses);
    }
  }
}

fn part1(contents: &String) -> Result<usize, String> {
  let mut processor = match parse_modules(contents) {
    Ok(processor) => processor,
    Err(error) => return Err(error),
  };

  let (high_signal_count, low_signal_count) = (0..1000)
    .fold((0, 0), |acc, _| {
      let (high_signal_count, low_signal_count) = process_signal(&mut processor, ("button", "broadcaster", false));

      (acc.0 + high_signal_count, acc.1 + low_signal_count)
    });

  Ok(high_signal_count * low_signal_count)
}

fn part2(contents: &String) -> Result<usize, String> {
  let mut processor = match parse_modules(contents) {
    Ok(processor) => processor,
    Err(error) => return Err(error),
  };

  let impulse_count = match find_minimum_impluses(&mut processor, "broadcaster", "rx") {
    Ok(impulse_count) => impulse_count,
    Err(error) => return Err(error),
  };

  Ok(impulse_count)
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