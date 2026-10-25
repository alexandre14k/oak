// src/root/options.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 25 oct 2026
// AGPL-3.0

use std::env;

#[derive(Debug)]
pub struct Options
{
  pub exclude: Vec<String>,
  pub types: Vec<String>,
  pub max_depth: usize,
  pub help: bool,
}

impl Default for Options
{
  fn default()
    -> Self
  {
    Self {
      exclude: Vec::new(),
      types: Vec::new(),
      max_depth: usize::MAX,
      help: false,
    }
  }
}

fn next_value(
  args: &mut impl Iterator<Item = String>,
  option: &str,
)
  -> Result<String, String>
{
  args.next()
    .ok_or_else(|| format!("Missing value for {option}"))
}

fn split_patterns(value: &str)
  -> Vec<String>
{
  value
    .split('|')
    .filter(|item| !item.is_empty())
    .map(str::to_string)
    .collect()
}

fn parse_types(value: &str)
  -> Vec<String>
{
  value
    .split('|')
    .filter(|item| !item.is_empty())
    .map(|item| {
      item.trim_start_matches('.').to_string()
    })
    .collect()
}

fn parse_depth(value: &str)
  -> Result<usize, String>
{
  let depth = value.parse::<usize>()
    .map_err(|_| {
      format!(
        "Invalid depth '{value}': expected a positive integer"
      )
    })?;

  if depth == 0
  {
    return Err("Depth must be at least 1".to_string());
  }

  Ok(depth)
}

fn apply_option(
  arg: &str,
  args: &mut impl Iterator<Item = String>,
  options: &mut Options,
)
  -> Result<(), String>
{
  match arg
  {
    "-h" | "--help" =>
    {
      options.help = true;
    }

    "-e" | "--exclude" =>
    {
      let value = next_value(args, arg)?;
      options.exclude.extend(split_patterns(&value));
    }

    "-t" | "--types" =>
    {
      let value = next_value(args, arg)?;
      options.types.extend(parse_types(&value));
    }

    "-r" | "--recursive" =>
    {
      let value = next_value(args, arg)?;
      options.max_depth = parse_depth(&value)?;
    }

    _ =>
    {
      return Err(format!(
        "Unknown argument: {arg}\n\
         Use 'oak -h' for help."
      ));
    }
  }

  Ok(())
}

pub fn parse()
  -> Result<Options, String>
{
  let mut options = Options::default();
  let mut args = env::args().skip(1);

  while let Some(arg) = args.next()
  {
    apply_option(&arg, &mut args, &mut options)?;
  }

  Ok(options)
}
