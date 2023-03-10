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

const FILTER: &[&str] = &[
    "GP_BORDER",
    "GP_LINEHORZ",
    "GP_LINEVERT",
    "MENU_POPUPITEM_FOCUSABLE",
    "MENU_POPUPITEMKBFOCUS",
    "SEBP_SEARCHEDITBOXTEXT",
    "TBP_SIZINGBARBOTTOMLEFT",
    "SEBTS_FORMATTED",
    "MPIF_NORMAL",
    "MPIKBFOCUS_NORMAL",
    "BSS_FLAT",
    "LHS_FLAT",
    "LVS_FLAT",
];

fn main() -> io::Result<()> {
    let mut cout = io::stdout().lock();

    writeln!(cout, "// SPDX-License-Identifier: BSL-1.0 OR Apache-2.0")?;
    writeln!(cout, "// Automatically generated, do not manually edit.")?;
    writeln!(cout)?;
    writeln!(cout, "use core::cmp::Ordering;")?;
    writeln!(cout, "use core::hash::{{Hash, Hasher}};")?;
    writeln!(cout)?;

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
                match row.len() {
                    2 | 3 => {
                        // Ignore the first entry if there are 3 entires.
                        if row.len() == 3 {
                            row.remove(0);
                        }

                        // Take the last part and put it into parts.
                        let new_part = Part {
                            name: extract_name(&row.remove(0)),
                            states: row[0].split(',').map(extract_name).collect(),
                        };
                        if let Some(part) = current_part.replace(new_part) {
                            parts.push(part);
                        }
                    }

                    1 => {
                        // Push this entry into the current part.
                        current_part
                            .as_mut()
                            .unwrap()
                            .states
                            .extend(row[0].split(',').map(extract_name))
                    }

                    len => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Unexpected length: {len}"),
                        ))
                    }
                }
            }

            parts.extend(current_part);

            break;
        }
    }

    // Remove the first two entries.
    for _ in 0..2 {
        parts.remove(0);
    }

    // Output the Part enum.
    writeln!(cout, "/// The part of the widget theme to retrieve.")?;
    writeln!(
        cout,
        "#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]"
    )?;
    writeln!(cout, "#[non_exhaustive]")?;
    writeln!(cout, "pub enum Part {{")?;

    for part in &parts {
        if FILTER.contains(&part.name.as_str()) {
            continue;
        }
        writeln!(cout, "    /// The `{}` part.", part.name)?;
        writeln!(cout, "    {0}({0}),", rust_part_name(&part.name))?;
    }

    writeln!(cout, "}}")?;
    writeln!(cout)?;

    // Output the individual enums.
    for part in &parts {
        if FILTER.contains(&part.name.as_str()) {
            continue;
        }
        writeln!(cout, "/// The state of the `{}` part.", part.name)?;
        writeln!(cout, "#[derive(Debug, Copy, Clone)]")?;
        writeln!(cout, "#[non_exhaustive]")?;
        writeln!(cout, "pub enum {} {{", rust_part_name(&part.name))?;
        writeln!(cout, "    /// The default state.")?;
        writeln!(cout, "    None,")?;

        for state in part.states.iter().filter(|s| !s.is_empty()) {
            if FILTER.contains(&state.as_str()) {
                continue;
            }

            writeln!(cout, "    /// The `{state}` state.")?;
            writeln!(cout, "    {0},", rust_part_name(state),)?;
        }

        writeln!(cout, "}}")?;
        writeln!(cout)?;

        // Output the `default` impl.
        writeln!(cout, "impl Default for {} {{", rust_part_name(&part.name))?;
        writeln!(cout, "    fn default() -> Self {{")?;
        writeln!(cout, "        Self::None")?;
        writeln!(cout, "    }}")?;
        writeln!(cout, "}}")?;
        writeln!(cout)?;

        // Output the PartialEq and Eq impls.
        writeln!(
            cout,
            "impl PartialEq<i32> for {} {{",
            rust_part_name(&part.name)
        )?;
        writeln!(cout, "    fn eq(&self, other: &i32) -> bool {{")?;
        writeln!(cout, "        self.state() == *other")?;
        writeln!(cout, "    }}")?;
        writeln!(cout, "}}")?;
        writeln!(cout)?;

        writeln!(cout, "impl PartialEq for {} {{", rust_part_name(&part.name))?;
        writeln!(cout, "    fn eq(&self, other: &Self) -> bool {{")?;
        writeln!(cout, "        self.state() == other.state()")?;
        writeln!(cout, "    }}")?;
        writeln!(cout, "}}")?;
        writeln!(cout)?;

        writeln!(cout, "impl Eq for {} {{}}", rust_part_name(&part.name))?;
        writeln!(cout)?;

        // Output the PartialOrd and Ord impls.
        writeln!(
            cout,
            "impl PartialOrd<i32> for {} {{",
            rust_part_name(&part.name)
        )?;
        writeln!(
            cout,
            "    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {{"
        )?;
        writeln!(cout, "        self.state().partial_cmp(other)")?;
        writeln!(cout, "    }}")?;
        writeln!(cout, "}}")?;
        writeln!(cout)?;

        writeln!(
            cout,
            "impl PartialOrd for {} {{",
            rust_part_name(&part.name)
        )?;
        writeln!(
            cout,
            "    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {{"
        )?;
        writeln!(cout, "        self.state().partial_cmp(&other.state())")?;
        writeln!(cout, "    }}")?;
        writeln!(cout, "}}")?;
        writeln!(cout)?;

        writeln!(cout, "impl Ord for {} {{", rust_part_name(&part.name))?;
        writeln!(cout, "    fn cmp(&self, other: &Self) -> Ordering {{")?;
        writeln!(cout, "        self.state().cmp(&other.state())")?;
        writeln!(cout, "    }}")?;
        writeln!(cout, "}}")?;
        writeln!(cout)?;

        // Output the `Hash` impl.
        writeln!(cout, "impl Hash for {} {{", rust_part_name(&part.name))?;
        writeln!(cout, "    fn hash<H: Hasher>(&self, state: &mut H) {{")?;
        writeln!(cout, "        self.state().hash(state);")?;
        writeln!(cout, "    }}")?;
        writeln!(cout, "}}")?;
        writeln!(cout)?;

        // Output the `state` method.
        writeln!(cout, "impl {} {{", rust_part_name(&part.name))?;
        writeln!(cout, "    /// Returns the state of this part.")?;
        writeln!(cout, "    fn state(self) -> i32 {{")?;
        writeln!(cout, "        match self {{")?;
        writeln!(cout, "            Self::None => 0,")?;

        for state in part.states.iter().filter(|s| !s.is_empty()) {
            if FILTER.contains(&state.as_str()) {
                continue;
            }

            writeln!(
                cout,
                "            Self::{0} => windows_sys::Win32::UI::Controls::{1},",
                rust_part_name(state),
                state,
            )?;
        }

        writeln!(cout, "        }}")?;
        writeln!(cout, "    }}")?;
        writeln!(cout, "}}")?;
        writeln!(cout)?;
    }

    // Output the `Part::part_and_state` method.
    writeln!(cout, "impl Part {{")?;
    writeln!(cout, "    /// Returns the part and state for this part.")?;
    writeln!(
        cout,
        "    pub(super) fn part_and_state(self) -> (i32, i32) {{"
    )?;
    writeln!(cout, "        match self {{")?;

    for part in &parts {
        if FILTER.contains(&part.name.as_str()) {
            continue;
        }
        writeln!(cout, "            Self::{0}(state) => (windows_sys::Win32::UI::Controls::{1}, state.state()),", rust_part_name(&part.name), part.name)?;
    }

    writeln!(cout, "        }}")?;
    writeln!(cout, "    }}")?;
    writeln!(cout, "}}")?;
    writeln!(cout)?;

    // Now, parse the property typedefs.
    let mut property_typedefs = {
        let typedefs_path = env::args_os()
            .nth(2)
            .unwrap_or_else(|| "property_typedefs.md".to_string().into());

        io::BufReader::new(File::open(typedefs_path)?)
    };

    // Skip the first ten lines.
    for _ in 0..10 {
        property_typedefs.read_line(&mut buf)?;
        buf.clear();
    }

    // Read the table of property types.
    let mut property_types = BTreeMap::new();
    loop {
        if let Some(table) = Table::parse(&mut property_typedefs)? {
            for row in table.rows.into_iter().skip(2) {
                // Should have four elements.
                if row.len() != 4 {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Unexpected length: {}", row.len()),
                    ));
                }

                let mut accessors = row[3]
                    .split(", ")
                    .map(extract_hyperlink)
                    .collect::<Vec<_>>();

                // Create a property type.
                let name = extract_name(&row[0]);
                let property_type = Property {
                    name: name.clone(),
                    accessor: accessors.remove(0),
                    sys_accessor: if accessors.is_empty() {
                        None
                    } else {
                        Some(accessors.remove(0))
                    },
                    subvalues: vec![(name.clone(), false)],
                };
                property_types.insert(name, property_type);
            }

            break;
        }
    }

    // Begin parsing the property tables.
    'outer: loop {
        // Look for a bold header.
        let header = loop {
            buf.clear();
            property_typedefs.read_line(&mut buf)?;
            if buf.is_empty() {
                break 'outer;
            }

            if buf.contains("**") {
                break extract_name(buf.split("**").nth(1).unwrap().trim());
            }
        };

        // Lookup in the property types.
        let property_type = property_types.get_mut(&header).unwrap();

        // Read in the table.
        loop {
            if let Some(table) = Table::parse(&mut property_typedefs)? {
                for row in table.rows.into_iter().skip(2) {
                    let mut name = extract_name(&row[0]);
                    if property_type.name.contains("ENUM") {
                        name = format!("TMT_{name}");
                    }

                    let is_sys = row.get(1).map_or(false, |s| s.contains("Sys"));
                    property_type.subvalues.push((name, is_sys));
                }

                break;
            }
        }
    }

    // Read out the types to write in methods.
    writeln!(cout, "impl crate::Theme {{")?;

    for property_type in property_types.values() {
        if property_type.name.contains("SIZE") {
            continue;
        }

        for (subvalue, is_sys) in &property_type.subvalues {
            writeln!(cout, "    /// Gets the `{subvalue}` property.")?;
            writeln!(cout, "    #[inline]")?;
            write!(cout, "    pub fn {}(&self", heck::AsSnakeCase(subvalue))?;

            if !is_sys {
                write!(cout, ", part: Part")?;
            }

            writeln!(cout, ") -> crate::{}Ret {{", &property_type.accessor)?;

            write!(
                cout,
                "        self.{}(",
                heck::AsSnakeCase(if *is_sys {
                    property_type.sys_accessor.as_ref().unwrap()
                } else {
                    &property_type.accessor
                })
            )?;

            if !is_sys {
                write!(cout, "part, ")?;
            }

            writeln!(cout, "windows_sys::Win32::UI::Controls::{})", &subvalue)?;

            writeln!(cout, "    }}")?;
            writeln!(cout)?;
        }
    }

    writeln!(cout, "}}")?;
    writeln!(cout)?;

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
                    let mut cells = line
                        .split('|')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<_>>();

                    if cells.get(0).map(|s| s.as_str()) == Some("") {
                        cells.remove(0);
                    }

                    if cells.last().map(|s| s.as_str()) == Some("") {
                        cells.pop();
                    }

                    cells
                })
            })
            .collect::<io::Result<Vec<_>>>()?;

        Ok(Some(Self { rows }))
    }
}

#[derive(Debug)]
struct Part {
    name: String,
    states: Vec<String>,
}

#[derive(Debug)]
struct Property {
    name: String,
    accessor: String,
    sys_accessor: Option<String>,
    subvalues: Vec<(String, bool)>,
}

fn extract_name(name: &str) -> String {
    let mut name = name
        .split(' ')
        .next()
        .unwrap()
        .split('*')
        .next()
        .unwrap()
        .trim()
        .to_string();

    name.retain(|c| c != '\\');

    name
}

fn extract_hyperlink(link: &str) -> String {
    link.split("[**")
        .nth(1)
        .unwrap()
        .split("**]")
        .next()
        .unwrap()
        .trim()
        .to_string()
}

fn rust_part_name(part: &str) -> impl std::fmt::Display + '_ {
    heck::AsUpperCamelCase(part)
}
