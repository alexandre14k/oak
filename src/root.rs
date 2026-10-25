// src/root.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 25 oct 2026
// AGPL-3.0

pub mod excludes;
pub mod help;
pub mod options;

use std::fs;
use std::path::{Path, PathBuf};

use options::Options;

struct Node
{
  name: String,
  is_dir: bool,
  children: Vec<Node>,
}

fn relative_string(path: &Path)
  -> String
{
  path.to_string_lossy().replace('\\', "/")
}

fn read_entry(
  entry: fs::DirEntry,
  relative: &Path,
  depth: usize,
  options: &Options,
  files: &mut Vec<Node>,
  directories: &mut Vec<Node>,
)
  -> Result<(), String>
{
  let path = entry.path();
  let name = entry.file_name().to_string_lossy().into_owned();
  let relative_path = relative.join(&name);
  let relative_name = relative_string(&relative_path);

  if excludes::is_excluded(
    &relative_name,
    &options.exclude,
  )
  {
    return Ok(());
  }

  let file_type = entry.file_type()
    .map_err(|e| {
      format!("Cannot inspect '{}': {e}", path.display())
    })?;

  add_entry(
    path.as_path(),
    name,
    relative_path,
    file_type,
    depth,
    options,
    files,
    directories,
  )
}

fn add_entry(
  path: &Path,
  name: String,
  relative: PathBuf,
  file_type: fs::FileType,
  depth: usize,
  options: &Options,
  files: &mut Vec<Node>,
  directories: &mut Vec<Node>,
)
  -> Result<(), String>
{
  if file_type.is_dir()
  {
    return add_directory(
      path,
      relative,
      name,
      depth,
      options,
      directories,
    );
  }

  if file_type.is_file()
    && excludes::matches_type(path, &options.types)
  {
    files.push(Node {
      name,
      is_dir: false,
      children: Vec::new(),
    });
  }

  Ok(())
}

fn add_directory(
  path: &Path,
  relative: PathBuf,
  name: String,
  depth: usize,
  options: &Options,
  directories: &mut Vec<Node>,
)
  -> Result<(), String>
{
  let children = build_tree(
    path,
    &relative,
    depth + 1,
    options,
  )?;

  if options.types.is_empty() || !children.is_empty()
  {
    directories.push(Node {
      name,
      is_dir: true,
      children,
    });
  }

  Ok(())
}

fn build_tree(
  directory: &Path,
  relative: &Path,
  depth: usize,
  options: &Options,
)
  -> Result<Vec<Node>, String>
{
  if depth >= options.max_depth
  {
    return Ok(Vec::new());
  }

  let entries = fs::read_dir(directory)
    .map_err(|e| {
      format!("Cannot read '{}': {e}", directory.display())
    })?;

  let mut files = Vec::new();
  let mut directories = Vec::new();

  for entry in entries
  {
    let entry = entry.map_err(|e| {
      format!("Cannot read directory entry: {e}")
    })?;

    read_entry(
      entry,
      relative,
      depth,
      options,
      &mut files,
      &mut directories,
    )?;
  }

  sort_nodes(&mut files);
  sort_nodes(&mut directories);

  files.extend(directories);

  Ok(files)
}

fn sort_nodes(nodes: &mut [Node])
{
  nodes.sort_by(|a, b| a.name.cmp(&b.name));
}

fn print_tree(nodes: &[Node], depth: usize)
{
  for node in nodes
  {
    print_node(node, depth);
  }
}

fn print_node(node: &Node, depth: usize)
{
  let indent = "  ".repeat(depth.saturating_sub(1));
  let suffix = if node.is_dir { "/" } else { "" };

  if depth == 0
  {
    println!("{}{suffix}", node.name);
  }
  else
  {
    println!("{indent}- {}{suffix}", node.name);
  }

  if node.is_dir
  {
    print_tree(&node.children, depth + 1);
  }
}

pub fn run()
  -> Result<(), String>
{
  let options = options::parse()?;

  if options.help
  {
    help::print_help();
    return Ok(());
  }

  let current_directory: PathBuf =
    std::env::current_dir()
      .map_err(|e| {
        format!("Cannot determine current directory: {e}")
      })?;

  let tree = build_tree(
    &current_directory,
    Path::new(""),
    0,
    &options,
  )?;

  println!(".");
  print_tree(&tree, 0);

  Ok(())
}
