use crossterm::{
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor, Stylize},
};
use std::fs;
use std::io;
use std::path::PathBuf;

const SYM_BRANCH: &str = "├── ";
const SYM_EMPTY: &str = "    ";
const SYM_LEAF: &str = "└── ";
const SYM_VERT: &str = "│   ";

/// Print the tree of the given path
pub fn print(path: PathBuf, header: String, is_last: bool, depth: u8) {
    // If not at the root, print the current path
    if depth > 0 {
        // Get the file name
        let mut file_name = path.file_name().unwrap().to_str().unwrap().stylize();

        // Determine the symbol to use
        let sym = if is_last { SYM_LEAF } else { SYM_BRANCH };

        // If the path is a directory, color it blue
        if path.is_dir() {
            file_name = file_name.with(Color::Blue);
        }

        // Print the current path
        execute!(
            io::stdout(),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Grey),
            Print(&header),
            Print(sym),
            ResetColor,
            Print(file_name),
            Print("\n"),
            ResetColor,
        )
        .ok();
    }
    // If root, print `.Pass`
    else {
        execute!(
            io::stdout(),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Red),
            Print("."),
            SetForegroundColor(Color::Green),
            Print("Pass"),
            Print("\n"),
            ResetColor
        )
        .ok();
    }

    // If it's not a directory, we're done here
    if !path.is_dir() {
        return;
    };

    // Get the children of the current path
    let mut children = fs::read_dir(&path)
        .expect("Could not read directory")
        .filter_map(|c| c.ok().map(|c| c.path()))
        // Filter out files starting with `.`, assumed to be hidden
        .filter(|c| {
            c.file_name()
                .is_some_and(|f| !f.to_string_lossy().starts_with("."))
        })
        .collect::<Vec<PathBuf>>();

    // Sort alphabetically
    children.sort();

    for (i, child) in children.iter().enumerate() {
        let child_is_last = i == children.len() - 1;
        let sym = if depth == 0 {
            ""
        } else if is_last {
            SYM_EMPTY
        } else {
            SYM_VERT
        };
        let child_header = format!("{}{}", header, sym);
        let child_depth = depth + 1;

        print(child.to_owned(), child_header, child_is_last, child_depth);
    }
}
