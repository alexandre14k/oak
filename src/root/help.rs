// src/root/help.rs
// alexandre raduly <alexander14k28@gmail.com>
// update 25 oct 2026
// AGPL-3.0

pub fn print_help()
{
  println!(
    r#"oak - display a recursive directory tree

USAGE:
  oak [OPTIONS]

OPTIONS:
  -h, --help
      Show this help prompt

  -e, --exclude LIST
      Exclude paths containing these patterns

  -t, --types LIST
      Include only files with these extensions

  -r, --recursive N
      Limit directory depth (default: unlimited)

FILTER FORMAT:
  Separate multiple values with '|'.

EXAMPLES:
  oak
  oak -h
  oak -e "src|target"
  oak -t "toml|rs"
  oak -r 3
  oak -t "toml|rs" -e "target" -r 3

NOTES:
  - Scanning starts from the current directory.
  - Hidden files are included.
  - Exclusion matching is case-sensitive.
  - Extension matching is case-sensitive.
  - Files are listed before directories.
  - Symbolic links are not recursively followed.
  - Depth 1 lists entries in the current directory.

USER:
  oak is a command-line directory tree listing tool.
  Use 'oak --help' to display this help message.

BUILD:
  version 1.0.0 on 25 oct 2026

AUTHOR:
  Written by Alexandre Raduly <alexander14k28@gmail.com>.

LICENSE:
  AGPL-3.0 license
  See the LICENSE file in the project root for
  licensing terms."#
  );
}
