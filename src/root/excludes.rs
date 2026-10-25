// src/root/excludes.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 25 oct 2026
// AGPL-3.0

use std::path::Path;

pub fn is_excluded(
  path: &str,
  patterns: &[String],
)
  -> bool
{
  patterns.iter().any(|pattern| {
    matches_pattern(pattern, path)
  })
}

fn matches_pattern(pattern: &str, path: &str)
  -> bool
{
  if !pattern.contains('*') && !pattern.contains('?')
  {
    return path.contains(pattern);
  }

  glob_matches(pattern, path)
}

fn glob_matches(pattern: &str, text: &str)
  -> bool
{
  let pattern: Vec<char> = pattern.chars().collect();
  let text: Vec<char> = text.chars().collect();

  let mut p = 0;
  let mut t = 0;
  let mut star = None;
  let mut retry = 0;

  while t < text.len()
  {
    if matches_char(&pattern, p, text[t])
    {
      p += 1;
      t += 1;
    }
    else if pattern.get(p) == Some(&'*')
    {
      star = Some(p);
      p += 1;
      retry = t;
    }
    else if let Some(star_index) = star
    {
      p = star_index + 1;
      retry += 1;
      t = retry;
    }
    else
    {
      return false;
    }
  }

  while pattern.get(p) == Some(&'*')
  {
    p += 1;
  }

  p == pattern.len()
}

fn matches_char(
  pattern: &[char],
  index: usize,
  character: char,
)
  -> bool
{
  match pattern.get(index)
  {
    Some('*') => false,
    Some('?') => true,
    Some(expected) => *expected == character,
    None => false,
  }
}

pub fn matches_type(
  path: &Path,
  types: &[String],
)
  -> bool
{
  if types.is_empty()
  {
    return true;
  }

  let Some(extension) = path
    .extension()
    .and_then(|ext| ext.to_str())
  else
  {
    return false;
  };

  types.iter().any(|item| item == extension)
}
