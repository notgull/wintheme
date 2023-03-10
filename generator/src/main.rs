// SPDX-License-Identifier: BSL-1.0 OR Apache-2.0
//               Copyright John Nunley, 2023.
// Distributed under the Boost Software License, Version 1.0 or the Apache
//                 License, Version 2.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*};

fn main() -> io::Result<()> {
    let mut cout = io::stdout().lock();

    // First, let's parse parts and states.
    // Open the parts_and_states.md file and start reading from it.
    let mut parts_and_states = {
        let parts_path = env::args_os()
            .nth(1)
            .unwrap_or_else(|| "parts_and_states.md".to_string().into());

        io::BufReader::new(File::open(parts_path)?)
    };

    // Skip the first ten lines.
    let mut buf = String::new();
    for _ in 0..10 {
        parts_and_states.read_line(&mut buf)?;
        buf.clear();
    }

    // Read until we find a table.
    let mut parts = vec![];
    loop {
        if let Some(table) = Table::parse(&mut parts_and_states)? {
            let mut current_part = None;

            // Read items from the table.
            for mut row in table.rows {
                writeln!(cout, "{:?}", &row)?;

                match row.len() {
                    2 | 3 => {
                        // Ignore the first entry if there are 3 entires.
                        if row.len() == 3 {
                            row.remove(0);
                        }

                        // Take the last part and put it into parts.
                        let new_part = Part {
                            name: row.remove(0),
                            states: row[0].split(',').map(|s| s.trim().to_string()).collect(),
                        };
                        if let Some(part) = current_part.replace(new_part) {
                            parts.push(part);
                        }
                    }

                    1 => {
                        // Push this entry into the current part.
                        current_part.states.extend(row[0].split(',').map(|s| s.trim().to_string()).collect());
                    }

                    len => return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Unexpected length: {len}")
                    ))
                }
            }

            parts.extend(current_part);

            break;
        }
    }

    Ok(())
}

struct Table {
    rows: Vec<Vec<String>>,
}

impl Table {
    fn parse(mut reader: impl BufRead) -> io::Result<Option<Self>> {
        // Read a line.
        let mut line = String::new();
        reader.read_line(&mut line)?;

        // If the line is empty, error out.
        if line.is_empty() {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }

        // If the line doesn't contain a `|`, bail out.
        let bars = line.chars().filter(|&c| c == '|').count();
        if bars == 0 {
            return Ok(None);
        }

        let rows = Some(Ok(line))
            .into_iter()
            .chain(reader.lines())
            .take_while(|line| line.as_ref().map_or(false, |line| line.contains('|')))
            .map(|line| {
                line.map(|line| {
                    line.split('|')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                })
            })
            .collect::<io::Result<Vec<_>>>()?;

        Ok(Some(Self { rows }))
    }
}

struct Part {
    name: String,
    states: Vec<String>,
}
